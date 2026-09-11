import { useState, useRef, useEffect, useMemo } from "react";
import { convertFileSrc } from "@tauri-apps/api/tauri";

interface Props {
  src: string | null;
  alt: string;
  className?: string;
  placeholder?: string;
}

export default function LazyImage({ src, alt, className = "", placeholder }: Props) {
  const [loaded, setLoaded] = useState(false);
  const [error, setError] = useState(false);
  const imgRef = useRef<HTMLImageElement>(null);

  const assetSrc = useMemo(() => {
    if (!src) return null;
    return convertFileSrc(src);
  }, [src]);

  useEffect(() => {
    setLoaded(false);
    setError(false);
  }, [src]);

  useEffect(() => {
    if (!imgRef.current) return;
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          const img = entry.target as HTMLImageElement;
          if (img.dataset.src) {
            img.src = img.dataset.src;
          }
          observer.disconnect();
        }
      },
      { rootMargin: "200px" }
    );
    observer.observe(imgRef.current);
    return () => observer.disconnect();
  }, []);

  if (!assetSrc || error) {
    return (
      <div className={`flex items-center justify-center ${className}`}
        style={{ background: "var(--evoury-elevated)" }}>
        <div className="text-center">
          <i className="fas fa-image text-lg" style={{ color: "var(--evoury-text-dim)" }} />
          <p className="text-[10px] mt-1" style={{ color: "var(--evoury-border-light)" }}>
            {error ? "Error" : placeholder || "No Preview"}
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className={`relative ${className}`} style={{ background: "var(--evoury-elevated)" }}>
      {!loaded && (
        <div className="absolute inset-0 flex items-center justify-center"
          style={{ background: "var(--evoury-elevated)" }}>
          <div className="panel-shimmer" style={{ position: "absolute", inset: 0 }} />
        </div>
      )}
      <img
        ref={imgRef}
        data-src={assetSrc}
        alt={alt}
        onLoad={() => setLoaded(true)}
        onError={() => setError(true)}
        className={`w-full h-full object-cover transition-opacity duration-500 ${
          loaded ? "opacity-100" : "opacity-0"
        }`}
      />
    </div>
  );
}
