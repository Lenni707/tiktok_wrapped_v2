import { useState, useEffect, useCallback } from "react";
import { slides } from "./Slides";
import "./slides.css";

export default function Slideshow({ user, onReset }) {
  const [idx, setIdx] = useState(0);
  // Bump this key to re-mount the slide wrapper → re-triggers CSS entrance animation
  const [mountKey, setMountKey] = useState(0);

  const goTo = useCallback((next) => {
    if (next < 0 || next >= slides.length) return;
    setIdx(next);
    setMountKey((k) => k + 1);
  }, []);

  const advance = useCallback(() => goTo(idx + 1), [idx, goTo]);
  const retreat = useCallback(() => goTo(idx - 1), [idx, goTo]);

  // Keyboard navigation
  useEffect(() => {
    const onKey = (e) => {
      if (e.key === "ArrowRight" || e.key === " ") { e.preventDefault(); advance(); }
      else if (e.key === "ArrowLeft")              { e.preventDefault(); retreat(); }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [advance, retreat]);

  const Slide = slides[idx];

  return (
    <div className="show-root" onClick={advance}>

      {/* ── Slide area ─────────────────────────────────── */}
      <div key={mountKey} className="show-slide-wrap">
        <Slide user={user} />
      </div>

      {/* ── Story-style progress segments ──────────────── */}
      <div className="show-progress" onClick={(e) => e.stopPropagation()}>
        {slides.map((_, i) => (
          <button
            key={i}
            className={`pb-seg ${
              i === idx   ? "pb-active" :
              i  <  idx   ? "pb-done"   : ""
            }`}
            onClick={() => goTo(i)}
            aria-label={`Go to slide ${i + 1}`}
          />
        ))}
      </div>

      {/* ── Left / Right nav hit areas ──────────────────── */}
      {idx > 0 && (
        <button
          className="show-nav show-nav-left"
          onClick={(e) => { e.stopPropagation(); retreat(); }}
          aria-label="Previous slide"
        >‹</button>
      )}
      {idx < slides.length - 1 && (
        <button
          className="show-nav show-nav-right"
          onClick={(e) => { e.stopPropagation(); advance(); }}
          aria-label="Next slide"
        >›</button>
      )}

      {/* ── Slide counter ──────────────────────────────── */}
      <div className="show-counter" aria-live="polite">
        {idx + 1} <span className="show-counter-sep">/</span> {slides.length}
      </div>

      {/* ── Reset (go back to upload screen) ───────────── */}
      <button
        className="show-reset"
        onClick={(e) => { e.stopPropagation(); onReset(); }}
        aria-label="Start over"
        title="Start over"
      >✕</button>

    </div>
  );
}
