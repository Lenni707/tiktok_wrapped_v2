import { useEffect, useRef, useState } from "react";

/**
 * Animated count-up number.
 * Props:
 *   from         – start value (default 0)
 *   to           – end value
 *   duration     – animation duration in seconds (default 1.8)
 *   separator    – thousands separator string (e.g. ",")
 *   decimals     – decimal places to display (default 0)
 *   prefix       – string prepended to value
 *   suffix       – string appended to value
 *   startCounting – set to true to trigger the animation
 */
export default function CountUp({
  from         = 0,
  to           = 0,
  duration     = 1.8,
  separator    = "",
  decimals     = 0,
  prefix       = "",
  suffix       = "",
  startCounting = true,
}) {
  const [val, setVal]   = useState(from);
  const rafRef          = useRef(null);
  const startTsRef      = useRef(null);

  useEffect(() => {
    if (!startCounting) {
      setVal(from);
      return;
    }

    startTsRef.current = null;
    cancelAnimationFrame(rafRef.current);

    const tick = (ts) => {
      if (!startTsRef.current) startTsRef.current = ts;

      const elapsed  = (ts - startTsRef.current) / 1000;
      const progress = Math.min(elapsed / duration, 1);

      // Ease-out exponential for snappy deceleration
      const eased = progress === 1 ? 1 : 1 - Math.pow(2, -10 * progress);
      setVal(from + (to - from) * eased);

      if (progress < 1) {
        rafRef.current = requestAnimationFrame(tick);
      }
    };

    rafRef.current = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(rafRef.current);
  }, [from, to, duration, startCounting]);

  // Format the number
  const display = (() => {
    const raw = decimals > 0 ? val.toFixed(decimals) : Math.floor(val).toString();
    if (!separator) return raw;
    const [intPart, decPart] = raw.split(".");
    const formatted = intPart.replace(/\B(?=(\d{3})+(?!\d))/g, separator);
    return decPart ? `${formatted}.${decPart}` : formatted;
  })();

  return <span>{prefix}{display}{suffix}</span>;
}
