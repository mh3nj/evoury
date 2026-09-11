<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>오프라인 중심 크리에이티브 자산 관리자</strong>
</p>

<p align="center">
  <a href="#features">기능</a> •
  <a href="#installation">설치</a> •
  <a href="#development">개발</a> •
  <a href="#architecture">아키텍처</a> •
  <a href="#contributing">기여</a> •
  <a href="#license">라이선스</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="버전">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="라이선스">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="플랫폼">
</p>

---

## 소개

Evoury는 Tauri, React, Rust로 구축된 강력한 오프라인 중심 크리에이티브 자산 관리자입니다. 성능이나 프라이버시를 타협하지 않고 디지털 자산에 빠르고 안정적으로 접근해야 하는 크리에이티브 전문가를 위해 설계되었습니다.

### 왜 Evoury인가?

- **오프라인 중심**: 자산이 로컬에 저장됩니다. 클라우드 의존성 없음.
- **초고속 성능**: Rust로 구축되어 라이브러리 규모에 따라 확장되는 성능.
- **모듈러 아키텍처**: 최대 유연성을 위한 40개 이상의 전용 크레이트.
- **아름다운 UI**: React와 Tailwind CSS로 구축된 모던하고 반응적인 인터페이스.

---

## 기능

### 핵심 엔진

- **멀티포맷 지원**: 이미지, 비디오, 3D 모델, 오디오, 문서 등
- **스마트 자산 페어링**: 관련 파일을 자동으로 그룹화
- **자산 상태 머신**: 발견, 검증, 인덱싱에서 아카이브까지의 생명주기 추적
- **이벤트 기반 아키텍처**: 이벤트 버스를 통한 서비스 간 비연결 통신

### 라이브러리 관리

- **고급 스캐너**: 전체, 증분, 폴더별, 백그라운드 스캔 모드
- **파일시스템 워처**: 수동 새로고침 없는 실시간 동기화
- **메타데이터 파이프라인**: 자동 추출, 정규화, 검증 및 캐싱
- **중복 감지**: SHA256, 지각적 해싱 및 메타데이터 기반 감지

### 검색 및 구성

- **영구 검색 인덱스**: FTS5를 사용한 초고속 전체 텍스트 검색
- **스마트 컬렉션**: 규칙 기반 자동 업데이트 컬렉션
- **고급 쿼리 언어**: 유형, 태그, 평점, 날짜, 카메라 등으로 필터링
- **검색 프로필**: 검색 구성 저장 및 전환

### 워크스페이스 시스템

- **영구 워크스페이스**: 전체 세션 상태를 기억
- **여러 워크스페이스**: 다른 프로젝트 컨텍스트 간 전환
- **워크스테이션**: 레이아웃, 도구, 단축키, 테마의 사전 구성
- **도킹 가능 패널**: 완전히 사용자 정의 가능한 레이아웃 엔진

### 건강 및 유지보수

- **건강 엔진**: 파일시스템, 데이터베이스, 캐시, 메타데이터 무결성 검사
- **자동 복구**: 감지된 문제에 대한 원클릭 복구
- **세션 복구**: 예상치 못한 종료 후 워크스페이스 복원
- **슬립 모드**: 유휴 시 최소 리소스 사용

---

## 스크린샷

<p align="center">
  <img src="docs/images/screenshot-main.png" alt="메인 인터페이스" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>메인 인터페이스 - 갤러리 뷰</em>
</p>

<p align="center">
  <img src="docs/images/screenshot-inspector.png" alt="인스펙터 패널" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>인스펙터 패널 - 자산 상세정보</em>
</p>

<p align="center">
  <img src="docs/images/screenshot-search.png" alt="검색 인터페이스" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>고급 검색 인터페이스</em>
</p>

---

## 설치

### 사전 요구사항

- [Rust](https://www.rust-lang.org/tools/install) (최신 안정 버전)
- [Node.js](https://nodejs.org/) (v18 이상)
- [pnpm](https://pnpm.io/) (v8 이상)

### 다운로드

[Releases](https://github.com/mh3nj/evoury/releases) 페이지에서 최신 버전을 다운로드하세요.

### 소스에서 빌드

```bash
# 저장소 클론
git clone https://github.com/mh3nj/evoury.git
cd evoury

# 의존성 설치
pnpm install

# 개발 서버 시작
pnpm tauri dev

# 프로덕션 빌드
pnpm tauri build
```

---

## 개발

### 사용 가능한 명령어

```bash
# 개발
pnpm dev              # Vite 개발 서버 시작
pnpm tauri dev        # 개발 모드로 Tauri 시작

# 빌드
pnpm build            # 프론트엔드 빌드
pnpm tauri build      # Tauri 앱 프로덕션 빌드

# 테스트
pnpm test             # 프론트엔드 테스트 실행
cargo test            # Rust 테스트 실행

# 린트
pnpm lint             # ESLint 실행
cargo clippy          # Clippy 실행

# 포맷
pnpm format           # 프론트엔드 코드 포맷
cargo fmt             # Rust 코드 포맷
```

---

## 기술 스택

### 백엔드

- **Rust** - 시스템 프로그래밍 언어
- **Tauri** - 데스크톱 애플리케이션 프레임워크
- **SQLite** - 로컬 데이터베이스
- **Crossbeam** - 동시성 프로그래밍 프리미티브

### 프론트엔드

- **React** - UI 라이브러리
- **TypeScript** - 타입 안전 JavaScript
- **Tailwind CSS** - 유틸리티 프리 CSS 프레임워크
- **Zustand** - 상태 관리
- **Vite** - 빌드 도구 및 개발 서버

---

## 로드맵

자세한 내용은 [ROADMAP.md](ROADMAP.md)를 참조하세요.

---

## 기여

기여는 환영합니다! 먼저 [CONTRIBUTING.md](CONTRIBUTING.md)를 읽어주세요.

---

## 라이선스

이 프로젝트는 MIT 라이선스로 라이선스가 부여됩니다. 자세한 내용은 [LICENSE](LICENSE) 파일을 참조하세요.

---

## 지원

- **이슈**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **토론**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  <a href="https://github.com/mh3nj">Your Name</a>이 ❤️로 제작
</p>
