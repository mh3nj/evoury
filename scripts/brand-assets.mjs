/**
 * One-off script: regenerate all web/logo assets from the Tauri-generated
 * src-tauri/icons/icon.png (512x512 master produced from evoury.png).
 *
 * Outputs:
 *   public/logo/logo.png                     (512x512)
 *   public/favicon/favicon.ico               (PNG entries: 16/32/48/64/128/256)
 *   public/favicon/favicon-16x16.png
 *   public/favicon/favicon-32x32.png
 *   public/favicon/apple-touch-icon.png      (180x180)
 *   public/favicon/android-chrome-192x192.png
 *   public/favicon/android-chrome-512x512.png
 *   docs/images/logo.png                     (512x512)
 *
 * The source icon is already transparent, so all outputs inherit transparency.
 * Run: node scripts/brand-assets.mjs
 */

import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { inflateSync, deflateSync } from "node:zlib";

// ---------------------------------------------------------------- CRC32 ----
const CRC_TABLE = new Int32Array(256);
for (let n = 0; n < 256; n++) {
  let c = n;
  for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
  CRC_TABLE[n] = c;
}
function crc32(buf) {
  let crc = -1;
  for (let i = 0; i < buf.length; i++) crc = (crc >>> 8) ^ CRC_TABLE[(crc ^ buf[i]) & 0xff];
  return (crc ^ -1) >>> 0;
}

// ----------------------------------------------------------- PNG decode ----
// Minimal decoder: 8-bit, non-interlaced, color type 6 (RGBA).
function decodePng(file) {
  const b = readFileSync(file);
  if (b.readUInt32BE(0) !== 0x89504e47) throw new Error(`${file}: not a PNG`);
  let off = 8;
  let w = 0,
    h = 0;
  const idat = [];
  while (off + 8 <= b.length) {
    const len = b.readUInt32BE(off);
    const type = b.toString("ascii", off + 4, off + 8);
    if (type === "IHDR") {
      w = b.readUInt32BE(off + 8);
      h = b.readUInt32BE(off + 12);
      const bitDepth = b[off + 16];
      const colorType = b[off + 17];
      const interlace = b[off + 20];
      if (bitDepth !== 8) throw new Error(`${file}: unsupported bit depth ${bitDepth}`);
      if (colorType !== 6) throw new Error(`${file}: unsupported color type ${colorType} (need RGBA)`);
      if (interlace !== 0) throw new Error(`${file}: interlaced PNG not supported`);
    } else if (type === "IDAT") {
      idat.push(b.subarray(off + 8, off + 8 + len));
    }
    off += 12 + len;
  }
  const raw = inflateSync(Buffer.concat(idat));
  const stride = w * 4;
  const px = Buffer.alloc(w * h * 4);
  let pos = 0;
  for (let y = 0; y < h; y++) {
    const filter = raw[pos++];
    const row = px.subarray(y * stride, (y + 1) * stride);
    const prev = y > 0 ? px.subarray((y - 1) * stride, y * stride) : null;
    for (let x = 0; x < stride; x++) {
      const rawByte = raw[pos + x];
      const a = x >= 4 ? row[x - 4] : 0;
      const bb = prev ? prev[x] : 0;
      const c = x >= 4 && prev ? prev[x - 4] : 0;
      let v;
      switch (filter) {
        case 0: v = rawByte; break;
        case 1: v = rawByte + a; break;
        case 2: v = rawByte + bb; break;
        case 3: v = rawByte + ((a + bb) >> 1); break;
        case 4: {
          const p = a + bb - c;
          const pa = Math.abs(p - a);
          const pb = Math.abs(p - bb);
          const pc = Math.abs(p - c);
          const pred = pa <= pb && pa <= pc ? a : pb <= pc ? bb : c;
          v = rawByte + pred;
          break;
        }
        default: throw new Error(`${file}: bad filter type ${filter}`);
      }
      row[x] = v & 0xff;
    }
    pos += stride;
  }
  return { w, h, data: px };
}

// ---------------------------------------------------------- PNG encode -----
function encodePng(w, h, rgba) {
  const chunk = (type, data) => {
    const len = Buffer.alloc(4);
    len.writeUInt32BE(data.length, 0);
    const t = Buffer.from(type, "ascii");
    const crc = Buffer.alloc(4);
    crc.writeUInt32BE(crc32(Buffer.concat([t, data])), 0);
    return Buffer.concat([len, t, data, crc]);
  };
  const ihdr = Buffer.alloc(13);
  ihdr.writeUInt32BE(w, 0);
  ihdr.writeUInt32BE(h, 4);
  ihdr[8] = 8; // bit depth
  ihdr[9] = 6; // color type: RGBA
  const stride = w * 4;
  const raw = Buffer.alloc((stride + 1) * h);
  for (let y = 0; y < h; y++) {
    raw[y * (stride + 1)] = 0; // filter type 0
    rgba.copy(raw, y * (stride + 1) + 1, y * stride, (y + 1) * stride);
  }
  const idat = deflateSync(raw, { level: 9 });
  return Buffer.concat([
    Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]),
    chunk("IHDR", ihdr),
    chunk("IDAT", idat),
    chunk("IEND", Buffer.alloc(0)),
  ]);
}

// ------------------------------------------------------------- resize ------
// Area-average (box) resample with fractional coverage and straight-alpha
// un-premultiplied color averaging. Downscale-only usage.
function resizeRGBA(src, sw, sh, dw, dh) {
  const out = Buffer.alloc(dw * dh * 4);
  const xMap = [];
  for (let dx = 0; dx < dw; dx++) xMap.push({ s: (dx * sw) / dw, e: ((dx + 1) * sw) / dw });
  for (let dy = 0; dy < dh; dy++) {
    const sy0 = (dy * sh) / dh;
    const sy1 = ((dy + 1) * sh) / dh;
    for (let dx = 0; dx < dw; dx++) {
      const { s: sx0, e: sx1 } = xMap[dx];
      let r = 0,
        g = 0,
        b = 0,
        a = 0,
        area = 0;
      for (let sy = Math.floor(sy0); sy < Math.min(sh, Math.ceil(sy1)); sy++) {
        const wy = Math.min(sy + 1, sy1) - Math.max(sy, sy0);
        if (wy <= 0) continue;
        for (let sx = Math.floor(sx0); sx < Math.min(sw, Math.ceil(sx1)); sx++) {
          const wx = Math.min(sx + 1, sx1) - Math.max(sx, sx0);
          if (wx <= 0) continue;
          const wgt = wx * wy;
          const i = (sy * sw + sx) * 4;
          const pa = src[i + 3] / 255;
          r += src[i] * pa * wgt;
          g += src[i + 1] * pa * wgt;
          b += src[i + 2] * pa * wgt;
          a += pa * wgt;
          area += wgt;
        }
      }
      const o = (dy * dw + dx) * 4;
      if (a > 0) {
        out[o] = Math.min(255, Math.round(r / a));
        out[o + 1] = Math.min(255, Math.round(g / a));
        out[o + 2] = Math.min(255, Math.round(b / a));
        out[o + 3] = Math.min(255, Math.round((a / area) * 255));
      }
    }
  }
  return out;
}

// ---------------------------------------------------------------- ICO ------
function buildIco(pngs) {
  const count = pngs.length;
  const header = Buffer.alloc(6);
  header.writeUInt16LE(0, 0);
  header.writeUInt16LE(1, 2);
  header.writeUInt16LE(count, 4);
  const entries = Buffer.alloc(16 * count);
  let offset = 6 + 16 * count;
  const blobs = [];
  pngs.forEach((p, i) => {
    const e = entries.subarray(i * 16, (i + 1) * 16);
    e[0] = p.size >= 256 ? 0 : p.size;
    e[1] = p.size >= 256 ? 0 : p.size;
    e[2] = 0; // palette
    e[3] = 0; // reserved
    e.writeUInt16LE(1, 4); // planes
    e.writeUInt16LE(32, 6); // bpp
    e.writeUInt32LE(p.buf.length, 8);
    e.writeUInt32LE(offset, 12);
    offset += p.buf.length;
    blobs.push(p.buf);
  });
  return Buffer.concat([header, entries, ...blobs]);
}

// ---------------------------------------------------------------- main -----
const master = decodePng("src-tauri/icons/icon.png");
if (master.w !== 512 || master.h !== 512) {
  throw new Error(`expected 512x512 master, got ${master.w}x${master.h}`);
}

function sized(size) {
  return size === 512 ? master.data : resizeRGBA(master.data, 512, 512, size, size);
}

const outputs = [
  { file: "public/logo/logo.png", size: 512 },
  { file: "public/favicon/favicon-16x16.png", size: 16 },
  { file: "public/favicon/favicon-32x32.png", size: 32 },
  { file: "public/favicon/apple-touch-icon.png", size: 180 },
  { file: "public/favicon/android-chrome-192x192.png", size: 192 },
  { file: "public/favicon/android-chrome-512x512.png", size: 512 },
  { file: "docs/images/logo.png", size: 512 },
];

mkdirSync("public/logo", { recursive: true });
mkdirSync("public/favicon", { recursive: true });
mkdirSync("docs/images", { recursive: true });

for (const { file, size } of outputs) {
  const png = encodePng(size, size, sized(size));
  writeFileSync(file, png);
  console.log(`wrote ${file} (${size}x${size}, ${png.length} bytes)`);
}

const icoSizes = [16, 32, 48, 64, 128, 256];
const ico = buildIco(icoSizes.map((size) => ({ size, buf: encodePng(size, size, sized(size)) })));
writeFileSync("public/favicon/favicon.ico", ico);
console.log(`wrote public/favicon/favicon.ico (${ico.length} bytes, entries: ${icoSizes.join(", ")})`);

console.log("done.");
