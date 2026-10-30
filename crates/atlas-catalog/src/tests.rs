use super::*;
use atlas_db::{connection::open_in_memory, MigrationRunner};
use std::fs;
use std::path::PathBuf;
#[cfg(unix)]
use std::time::{Duration, SystemTime};

struct TempDir(PathBuf);

impl TempDir {
    fn new(label: &str) -> Self {
        let dir = std::env::temp_dir().join(format!(
            "atlas-catalog-{label}-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        TempDir(dir)
    }
    fn put(&self, rel: &str, bytes: &[u8]) -> PathBuf {
        let p = self.0.join(rel);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(&p, bytes).unwrap();
        p
    }
    fn touch(&self, rel: &str) -> PathBuf {
        self.put(rel, rel.as_bytes())
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn new_db() -> Arc<Mutex<Connection>> {
    let conn = open_in_memory().unwrap();
    MigrationRunner::run(&conn).unwrap();
    Arc::new(Mutex::new(conn))
}

fn catalog_on(db: &Arc<Mutex<Connection>>) -> Catalog {
    Catalog::new(Arc::new(Library::new()), db.clone(), None)
}

#[cfg(unix)]
fn backdate(root: &Path) {
    let old = SystemTime::now() - Duration::from_secs(3600);
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        fs::File::open(&dir).unwrap().set_modified(old).unwrap();
        for e in fs::read_dir(&dir).unwrap().flatten() {
            if e.file_type().unwrap().is_dir() {
                stack.push(e.path());
            }
        }
    }
}

fn names(c: &Catalog) -> Vec<String> {
    let mut v: Vec<_> = c.library().all().into_iter().map(|a| a.name).collect();
    v.sort();
    v
}

// ── persistence ──────────────────────────────────────────────

#[test]
fn the_library_comes_back_after_a_restart_without_scanning() {
    let t = TempDir::new("restart");
    t.touch("Fonts/Logo.7z");
    t.touch("Fonts/Logo.avif");
    t.touch("Fonts/Logo.thumb.avif");
    t.touch("Mockups/Cards.rar");
    let db = new_db();

    let first = catalog_on(&db);
    let (root, summary) = first.add_and_scan(&t.0, ScanMode::FullScan).unwrap();
    assert_eq!((summary.added, summary.total), (2, 2));
    let before = {
        let mut v = first.library().all();
        v.sort_by(|a, b| a.archive_path.cmp(&b.archive_path));
        v
    };

    // "Restart": a brand new in-memory library on the same database. No scan happens.
    let second = catalog_on(&db);
    assert_eq!(second.library().len(), 0);
    assert_eq!(second.load_from_index().unwrap(), 2);
    let mut after = second.library().all();
    after.sort_by(|a, b| a.archive_path.cmp(&b.archive_path));
    assert_eq!(before.len(), after.len());
    for (a, b) in before.iter().zip(&after) {
        assert_eq!(
            (a.id, &a.name, a.file_size, a.root_id),
            (b.id, &b.name, b.file_size, b.root_id)
        );
        assert_eq!(
            a.preview.as_ref().map(|p| &p.path),
            b.preview.as_ref().map(|p| &p.path)
        );
        assert_eq!(
            a.preview.as_ref().and_then(|p| p.thumb.clone()),
            b.preview.as_ref().and_then(|p| p.thumb.clone())
        );
    }
    assert_eq!(second.roots().unwrap()[0].id, root.id);
    assert_eq!(second.roots().unwrap()[0].asset_count, 2);
}

#[test]
fn collections_and_categories_survive_rescans_and_restarts() {
    let t = TempDir::new("assign");
    t.touch("a.7z");
    let db = new_db();
    let c = catalog_on(&db);
    c.add_and_scan(&t.0, ScanMode::FullScan).unwrap();
    let id = c.library().all()[0].id;
    let (col, cat) = (Uuid::from_u128(11), Uuid::from_u128(22));
    c.set_assignment(id, Some(col), Some(cat)).unwrap();

    let root = c.roots().unwrap()[0].id;
    c.scan_root(root, ScanMode::FullScan).unwrap();
    let a = c.library().get(&id).unwrap();
    assert_eq!((a.collection_id, a.category_id), (Some(col), Some(cat)));

    let again = catalog_on(&db);
    again.load_from_index().unwrap();
    let a = again.library().get(&id).unwrap();
    assert_eq!((a.collection_id, a.category_id), (Some(col), Some(cat)));
}

#[cfg(unix)]
#[test]
fn after_a_restart_the_incremental_scan_only_lists_what_changed() {
    let t = TempDir::new("incr");
    t.touch("a/one.7z");
    t.touch("b/two.7z");
    t.touch("c/three.7z");
    backdate(&t.0);
    let db = new_db();
    let first = catalog_on(&db);
    let (root, _) = first.add_and_scan(&t.0, ScanMode::FullScan).unwrap();

    t.touch("b/four.7z");
    let second = catalog_on(&db);
    second.load_from_index().unwrap();
    let s = second.scan_root(root.id, ScanMode::StartupScan).unwrap();
    assert_eq!(names(&second), vec!["four", "one", "three", "two"]);
    assert_eq!((s.added, s.updated), (1, 3));
    assert_eq!(s.read_folders, 1, "only b/ changed");
    assert_eq!(s.reused_folders, 3);
}

#[cfg(unix)]
#[test]
fn deleting_files_is_noticed_and_remembered() {
    let t = TempDir::new("delete");
    t.touch("a/one.7z");
    t.touch("a/two.7z");
    backdate(&t.0);
    let db = new_db();
    let c = catalog_on(&db);
    let (root, _) = c.add_and_scan(&t.0, ScanMode::FullScan).unwrap();
    fs::remove_file(t.0.join("a/two.7z")).unwrap();
    let s = c.scan_root(root.id, ScanMode::IncrementalScan).unwrap();
    assert_eq!(s.removed, 1);
    let restarted = catalog_on(&db);
    restarted.load_from_index().unwrap();
    assert_eq!(names(&restarted), vec!["one"]);
}

// ── library folders ──────────────────────────────────────────

#[test]
fn two_libraries_with_the_same_layout_do_not_collide() {
    let a = TempDir::new("lib-a");
    let b = TempDir::new("lib-b");
    a.touch("Fonts/Logo.7z");
    b.touch("Fonts/Logo.7z");
    b.touch("Other/Thing.zip");
    let db = new_db();
    let c = catalog_on(&db);
    let (ra, _) = c.add_and_scan(&a.0, ScanMode::FullScan).unwrap();
    let (rb, _) = c.add_and_scan(&b.0, ScanMode::FullScan).unwrap();
    assert!(ra.id.is_nil() && !rb.id.is_nil());
    assert_eq!(c.library().len(), 3);
    let logos: Vec<_> = c
        .library()
        .all()
        .into_iter()
        .filter(|x| x.name == "Logo")
        .collect();
    assert_eq!(logos.len(), 2);
    assert_ne!(logos[0].id, logos[1].id);

    // rescanning one library leaves the other alone
    fs::remove_file(b.0.join("Other/Thing.zip")).unwrap();
    c.scan_root(rb.id, ScanMode::FullScan).unwrap();
    assert_eq!(c.library().len(), 2);
    assert_eq!(c.library().by_root(ra.id).len(), 1);

    let infos = c.roots().unwrap();
    assert_eq!(
        infos.iter().map(|r| r.asset_count).collect::<Vec<_>>(),
        vec![1, 1]
    );
}

#[test]
fn removing_a_library_removes_only_its_assets_and_never_its_files() {
    let a = TempDir::new("rm-a");
    let b = TempDir::new("rm-b");
    let file = a.touch("x/one.7z");
    b.touch("y/two.7z");
    let db = new_db();
    let c = catalog_on(&db);
    let (ra, _) = c.add_and_scan(&a.0, ScanMode::FullScan).unwrap();
    c.add_and_scan(&b.0, ScanMode::FullScan).unwrap();
    assert_eq!(c.remove_root(ra.id).unwrap(), 1);
    assert_eq!(names(&c), vec!["two"]);
    assert!(file.exists());
    let restarted = catalog_on(&db);
    restarted.load_from_index().unwrap();
    assert_eq!(names(&restarted), vec!["two"]);
    assert_eq!(restarted.roots().unwrap().len(), 1);
}

#[test]
fn overlapping_libraries_are_refused() {
    let t = TempDir::new("overlap");
    t.touch("inner/a.7z");
    let db = new_db();
    let c = catalog_on(&db);
    c.add_root(&t.0.join("inner"), None).unwrap();
    assert!(c
        .add_root(&t.0, None)
        .unwrap_err()
        .contains("contains the library"));
    let outer = TempDir::new("overlap2");
    outer.touch("sub/b.7z");
    c.add_root(&outer.0, None).unwrap();
    assert!(c
        .add_root(&outer.0.join("sub"), None)
        .unwrap_err()
        .contains("already part of"));
    // adding the same folder again is not an error, it is the same library
    assert_eq!(c.add_root(&outer.0, None).unwrap().path, outer.0);
    assert_eq!(c.roots().unwrap().len(), 2);
}

#[test]
fn adding_something_that_is_not_a_folder_fails_cleanly() {
    let t = TempDir::new("notdir");
    let file = t.touch("a.7z");
    let c = catalog_on(&new_db());
    assert!(c.add_root(&file, None).is_err());
    assert!(c.add_root(&t.0.join("missing"), None).is_err());
}

#[test]
fn an_unplugged_library_is_reported_and_never_emptied() {
    let keep = TempDir::new("online");
    let gone = TempDir::new("offline");
    keep.touch("a.7z");
    gone.touch("b.7z");
    gone.touch("c.7z");
    let db = new_db();
    let c = catalog_on(&db);
    c.add_and_scan(&keep.0, ScanMode::FullScan).unwrap();
    let (offline, _) = c.add_and_scan(&gone.0, ScanMode::FullScan).unwrap();
    assert_eq!(c.library().len(), 3);

    fs::remove_dir_all(&gone.0).unwrap(); // the drive disappears
    let err = c.scan_root(offline.id, ScanMode::FullScan).unwrap_err();
    assert!(err.contains("not available"), "{err}");
    let all = c.scan_all(ScanMode::StartupScan).unwrap();
    assert_eq!(all.errors.len(), 1);
    assert_eq!(
        c.library().len(),
        3,
        "the offline library's assets must stay"
    );
    let infos = c.roots().unwrap();
    assert!(infos[0].available && !infos[1].available);
    assert_eq!(infos[1].asset_count, 2);

    let restarted = catalog_on(&db);
    assert_eq!(restarted.load_from_index().unwrap(), 3);
}

#[test]
fn renaming_a_library() {
    let t = TempDir::new("rename");
    let c = catalog_on(&new_db());
    let r = c.add_root(&t.0, None).unwrap();
    c.rename_root(r.id, "  Client work ").unwrap();
    assert_eq!(c.roots().unwrap()[0].name, "Client work");
    assert!(c.rename_root(r.id, "   ").is_err());
}

// ── duplicates ───────────────────────────────────────────────

#[test]
fn byte_identical_archives_are_found_and_hashes_are_cached() {
    let t = TempDir::new("exact");
    let same = vec![0x5Au8; 40_000];
    let mut other = same.clone();
    other[39_999] ^= 1; // same size, different content
    t.put("one/Pack.7z", &same);
    t.put("two/Pack copy.7z", &same);
    t.put("three/Different.7z", &other);
    t.put("four/Small.7z", b"tiny");
    let db = new_db();
    let c = catalog_on(&db);
    c.add_and_scan(&t.0, ScanMode::FullScan).unwrap();

    let groups = c.find_exact_duplicates(&|| false).unwrap();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].assets.len(), 2);
    assert_eq!(groups[0].wasted_bytes(), 40_000);
    let a = c.library().get(&groups[0].assets[0]).unwrap();
    assert!(a.name.starts_with("Pack"));

    // hashes were cached; a second run gives the same answer
    let id = groups[0].assets[0];
    assert!(index::get_archive_hash(&db.lock().unwrap(), id)
        .unwrap()
        .unwrap()
        .full
        .is_some());
    assert_eq!(c.find_exact_duplicates(&|| false).unwrap(), groups);
}

#[test]
fn a_cancelled_duplicate_search_says_so() {
    let t = TempDir::new("cancel");
    let c = catalog_on(&new_db());
    t.put("a.7z", &[1u8; 100]);
    t.put("b.7z", &[1u8; 100]);
    c.add_and_scan(&t.0, ScanMode::FullScan).unwrap();
    assert_eq!(c.find_exact_duplicates(&|| true).unwrap_err(), "Cancelled.");
}

/// A fake archive plus a Kaelovun manifest for it: lets the content analysis run with
/// no external tool.
fn package(t: &TempDir, rel: &str, files: &[(&str, u64, u32)]) {
    let archive = t.put(rel, format!("archive:{rel}").as_bytes());
    let name = archive.file_name().unwrap().to_string_lossy().into_owned();
    let list: Vec<String> = files
        .iter()
        .map(|(p, s, c)| format!(r#"{{"path":"{p}","size":{s},"crc32":"{c:08X}"}}"#))
        .collect();
    let manifest = format!(
        r#"{{"archive":"{name}","archive_bytes":{},"files":[{}]}}"#,
        fs::metadata(&archive).unwrap().len(),
        list.join(",")
    );
    fs::write(
        archive.with_file_name(format!("{name}.kaelovun.json")),
        manifest,
    )
    .unwrap();
}

#[test]
fn shared_files_and_near_identical_packages_are_found_from_manifests() {
    let t = TempDir::new("shared");
    let font = ("fonts/Inter.ttf", 300_000, 0xAAAA);
    package(
        &t,
        "p1/Pack A.7z",
        &[font, ("a.psd", 90_000, 1), ("b.png", 20_000, 2)],
    );
    package(
        &t,
        "p2/Pack A (rar).rar",
        &[font, ("a.psd", 90_000, 1), ("b.png", 20_000, 2)],
    );
    package(
        &t,
        "p3/Mockup.zip",
        &[("assets/Inter.ttf", 300_000, 0xAAAA), ("m.psd", 70_000, 9)],
    );
    package(&t, "p4/Unrelated.7z", &[("z.psd", 80_000, 77)]);
    let db = new_db();
    let c = catalog_on(&db);
    c.add_and_scan(&t.0, ScanMode::FullScan).unwrap();

    let p = c.analyze_contents(100, &|| false).unwrap();
    assert_eq!((p.analyzed, p.failed, p.remaining), (4, 0, 0));
    // analysing again does nothing: it is cached
    assert_eq!(c.analyze_contents(100, &|| false).unwrap().analyzed, 0);

    let report = c
        .duplicate_report(&SharedOptions::default(), &|| false)
        .unwrap();
    assert_eq!((report.analyzed, report.total), (4, 4));
    assert!(report.exact.is_empty());

    let font_shared = report
        .shared_files
        .iter()
        .find(|f| f.size == 300_000)
        .unwrap();
    assert_eq!(font_shared.assets.len(), 3);
    assert_eq!(font_shared.wasted_bytes(), 600_000);

    let id_of = |n: &str| {
        c.library()
            .all()
            .into_iter()
            .find(|a| a.name == n)
            .unwrap()
            .id
    };
    let (a, b) = (id_of("Pack A"), id_of("Pack A (rar)"));
    let pair = report
        .similar
        .iter()
        .find(|s| (s.a == a && s.b == b) || (s.a == b && s.b == a))
        .expect("the same pack saved twice must be reported");
    assert!(pair.identical);
    assert_eq!(pair.shared_files, 3);
    assert!(report
        .similar
        .iter()
        .all(|s| s.a != id_of("Unrelated") && s.b != id_of("Unrelated")));
}

#[test]
fn analysis_works_in_batches_and_reports_what_is_left() {
    let t = TempDir::new("batches");
    for i in 0..5 {
        package(&t, &format!("p{i}/P{i}.7z"), &[("f.bin", 5_000, i)]);
    }
    let c = catalog_on(&new_db());
    c.add_and_scan(&t.0, ScanMode::FullScan).unwrap();
    let first = c.analyze_contents(2, &|| false).unwrap();
    assert_eq!((first.analyzed, first.remaining), (2, 3));
    let second = c.analyze_contents(10, &|| false).unwrap();
    assert_eq!((second.analyzed, second.remaining), (3, 0));
}

#[test]
fn a_changed_archive_is_analysed_again_and_a_removed_one_is_forgotten() {
    let t = TempDir::new("changed");
    package(&t, "a/One.7z", &[("f.bin", 5_000, 1)]);
    package(&t, "b/Two.7z", &[("f.bin", 5_000, 1)]);
    let db = new_db();
    let c = catalog_on(&db);
    let (root, _) = c.add_and_scan(&t.0, ScanMode::FullScan).unwrap();
    c.analyze_contents(10, &|| false).unwrap();
    assert_eq!(
        c.duplicate_report(&SharedOptions::default(), &|| false)
            .unwrap()
            .analyzed,
        2
    );

    // One.7z is replaced by a different file
    package(&t, "a/One.7z", &[("g.bin", 9_999, 5), ("h.bin", 1_111, 6)]);
    fs::write(t.0.join("a/One.7z"), b"completely different bytes").unwrap();
    let archive = t.0.join("a/One.7z");
    let manifest = format!(
        r#"{{"archive":"One.7z","archive_bytes":{},"files":[{{"path":"g.bin","size":9999,"crc32":"00000005"}}]}}"#,
        fs::metadata(archive).unwrap().len()
    );
    fs::write(t.0.join("a/One.7z.kaelovun.json"), manifest).unwrap();
    c.scan_root(root.id, ScanMode::FullScan).unwrap();
    let p = c.analyze_contents(10, &|| false).unwrap();
    assert_eq!(p.analyzed, 1);

    fs::remove_file(t.0.join("b/Two.7z")).unwrap();
    c.scan_root(root.id, ScanMode::FullScan).unwrap();
    assert_eq!(
        c.duplicate_report(&SharedOptions::default(), &|| false)
            .unwrap()
            .analyzed,
        1
    );
}

#[test]
fn archives_that_cannot_be_read_are_marked_and_not_retried() {
    let t = TempDir::new("unreadable");
    t.put("a/Broken.7z", b"not an archive at all");
    let c = catalog_on(&new_db());
    c.add_and_scan(&t.0, ScanMode::FullScan).unwrap();
    let first = c.analyze_contents(10, &|| false).unwrap();
    if atlas_scanner::contents::find_seven_zip().is_some() {
        // 7-Zip refuses it: counted as failed once, then left alone
        assert_eq!(first.failed, 1);
        let second = c.analyze_contents(10, &|| false).unwrap();
        assert_eq!(
            (second.analyzed, second.failed, second.remaining),
            (0, 0, 0)
        );
    } else {
        assert_eq!((first.needs_tool, first.remaining), (1, 1));
    }
}
