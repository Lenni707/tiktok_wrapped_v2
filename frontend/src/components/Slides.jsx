import { useState, useEffect } from "react";
import CountUp from "./CountUp";

// // ─── Helpers ──────────────────────────────────────────────────────────────────

// /**
//  * Parse a Rust `time::Duration` (serde JSON: { seconds, nanoseconds })
//  * into plain seconds. Handles raw numbers too.
//  */
// function parseDurSecs(d) {
//   if (!d) return 0;
//   if (typeof d === "number") return d;
//   if (typeof d === "object") {
//     const s = Number(d.seconds ?? d.secs   ?? 0);
//     const n = Number(d.nanoseconds ?? d.nanos ?? 0);
//     return s + n / 1e9;
//   }
//   return 0;
// }

// /**
//  * Format a date string (YYYY-MM-DD or ISO datetime) into "March 15" etc.
//  */
// function fmtDate(raw) {
//   if (!raw) return "Unknown";
//   try {
//     // Strip time portion if present so Date doesn't apply UTC offset weirdness
//     const dateOnly = String(raw).split("T")[0];
//     const d = new Date(dateOnly + "T12:00:00");
//     return d.toLocaleDateString("en-US", { month: "long", day: "numeric" });
//   } catch {
//     return String(raw);
//   }
// }

/**
 * Small hook: triggers `started = true` after `delay` ms.
 * Used to kick off CountUp animations after the slide entrance animation.
 */
function useStarted(delay = 450) {
  const [started, setStarted] = useState(false);
  useEffect(() => {
    const t = setTimeout(() => setStarted(true), delay);
    return () => clearTimeout(t);
  }, []);
  return started;
}

// ─── Slides ───────────────────────────────────────────────────────────────────

/* 1. Welcome ---------------------------------------------------------------- */
export function WelcomeSlide({ user }) {
  const started = useStarted(150);
  const { profile } = user;
  const pfpUrl = `/tiktok_pfp?url=${encodeURIComponent(profile.pfp)}`

  return (
    <div className="slide slide-welcome">
      <div className="slide-noise" aria-hidden />
      <div className="welcome-blobs" aria-hidden>
        <div className="wb wb1" /><div className="wb wb2" /><div className="wb wb3" />
      </div>

      <div className={`welcome-content ${started ? "wc-show" : ""}`}>
        {pfpUrl && (
          <div className="welcome-pfp-ring">
            <img src={pfpUrl} alt={profile.name} className="welcome-pfp" />
          </div>
        )}
        <p className="welcome-eyebrow">@{profile.name}</p>
        <h1 className="welcome-headline">
          Your<br />TikTok<br />Wrapped
        </h1>
        {profile.follower_count > 0 && (
          <p className="welcome-followers">
            <strong>{profile.follower_count.toLocaleString()}</strong> followers
          </p>
        )}
        <p className="welcome-cta">Tap anywhere to continue →</p>
      </div>
    </div>
  );
}

/* 2. Watch Time ------------------------------------------------------------- */
export function WatchTimeSlide({ user }) {
  const started  = useStarted();
  const totalSecs = user.activity.watch_time_secs;
  const days      = totalSecs / 86400;
  const hours     = Math.round(totalSecs / 3600);

  return (
    <div className="slide slide-watchtime">
      <div className="slide-noise" aria-hidden />
      <div className="stat-layout">
        <p className="stat-eyebrow">This year, you spent</p>
        <div className="stat-hero">
          <span className="stat-number">
            <CountUp from={0} to={days} duration={2.2} decimals={1} startCounting={started} />
          </span>
          <span className="stat-unit">days</span>
        </div>
        <p className="stat-body">glued to TikTok</p>
        <p className="stat-subtext">
          That's{" "}
          <strong>
            <CountUp from={0} to={hours} separator="," duration={1.8} startCounting={started} />
          </strong>{" "}
          hours of video
        </p>
        <span className="stat-deco" aria-hidden>🕰️</span>
      </div>
    </div>
  );
}

/* 3. Videos Watched --------------------------------------------------------- */
export function VideosSlide({ user }) {
  const started = useStarted();
  const count   = user.activity.vids_watched;
  const avgSecs = count / 365;

  return (
    <div className="slide slide-videos">
      <div className="slide-noise" aria-hidden />
      <div className="stat-layout">
        <p className="stat-eyebrow">You watched</p>
        <div className="stat-hero">
          <span className="stat-number">
            <CountUp from={0} to={count} separator="," duration={2.2} startCounting={started} />
          </span>
          <span className="stat-unit">videos</span>
        </div>
        <p className="stat-body">front to back</p>
        <p className="stat-subtext">
          That are <strong>{avgSecs.toFixed(0)}</strong> videos per day on avergae
        </p>
        <span className="stat-deco" aria-hidden>📱</span>
      </div>
    </div>
  );
}

/* 4. Watch Sessions --------------------------------------------------------- */
export function SessionsSlide({ user }) {
  const started  = useStarted();
  const sessions = user.activity.num_watch_sessions_one_year;
  const perDay   = (sessions / 365).toFixed(1);

  return (
    <div className="slide slide-sessions">
      <div className="slide-noise" aria-hidden />
      <div className="stat-layout">
        <p className="stat-eyebrow">You opened TikTok</p>
        <div className="stat-hero">
          <span className="stat-number">
            <CountUp from={0} to={sessions} separator="," duration={2.2} startCounting={started} />
          </span>
          <span className="stat-unit">times</span>
        </div>
        <p className="stat-body">this year</p>
        <p className="stat-subtext">
          That's <strong>{perDay}</strong> sessions a day on average
        </p>
        <span className="stat-deco" aria-hidden>🔁</span>
      </div>
    </div>
  );
}

/* 5. Longest Session -------------------------------------------------------- */
export function LongestSessionSlide({ user }) {
  const started   = useStarted();
  const session   = user.activity.longest_watch_session;
  const durSecs   = session?.duration_as_secs;
  const durHours  = durSecs / 3600;
  const vidsInSesh = session?.session.vids_watched ?? 0;
  const sessionDate = session?.date_as_string;

  return (
    <div className="slide slide-binge">
      <div className="slide-noise" aria-hidden />
      <div className="stat-layout">
        <p className="stat-eyebrow">Your longest binge</p>
        <div className="stat-hero">
          <span className="stat-number">
            <CountUp from={0} to={durHours} duration={2.2} decimals={1} startCounting={started} />
          </span>
          <span className="stat-unit">hours straight</span>
        </div>
        <p className="stat-body">without stopping</p>
        <p className="stat-subtext">
          {sessionDate && <><strong>{sessionDate}</strong> · </>}
          <strong>{vidsInSesh.toLocaleString()}</strong> videos watched
        </p>
        <span className="stat-deco" aria-hidden>😨</span>
      </div>
    </div>
  );
}

/* 6. Best Day --------------------------------------------------------------- */
export function BestDaySlide({ user }) {
  const started = useStarted(300);

  const sessionDate = user.activity.most_watch_sessions_per_day.date_as_string ?? ["??"];
  const sessionCount = user.activity.most_watch_sessions_per_day.count ?? [0];

  const timeDate = user.activity.most_time_spend_on_tiktok_day.date_as_string ?? ["?"];
  const timeDur = user.activity.most_time_spend_on_tiktok_day.duration_as_secs ?? [0.0];
  const timeHours = timeDur / 3600;

  return (
    <div className="slide slide-bestday">
      <div className="slide-noise" aria-hidden />
      <div className="stat-layout">
        <p className="stat-eyebrow">Your most active day was</p>
        <div className="bestday-date">{sessionDate}</div>
        <div className="bestday-split">
          <div className="bs-item">
            <span className="bs-num">
              <CountUp from={0} to={sessionCount} duration={1.6} startCounting={started} />
            </span>
            <span className="bs-label">watch sessions</span>
          </div>
          <div className="bs-divider" />
          <div className="bs-item">
            <span className="bs-num">
              <CountUp from={0} to={timeHours} duration={1.6} decimals={1} startCounting={started} />
              <span className="bs-unit">h</span>
            </span>
            <span className="bs-label">on TikTok</span>
          </div>
        </div>
        <span className="stat-deco" aria-hidden>📅</span>
      </div>
    </div>
  );
}

/* 7. Weekday Breakdown ------------------------------------------------------ */
export function WeekdaySlide({ user }) {
  const started = useStarted(300);
  const wk      = user.activity.avergae_time_per_weekday;

  const days = [
    { label: "Mon", value: wk.monday    },
    { label: "Tue", value: wk.tuesday   },
    { label: "Wed", value: wk.wednesday },
    { label: "Thu", value: wk.thursday  },
    { label: "Fri", value: wk.friday    },
    { label: "Sat", value: wk.saturday  },
    { label: "Sun", value: wk.sunday    },
  ];

  const maxVal      = Math.max(...days.map((d) => d.value), 1);
  const highestDay  = (wk.highest_day ?? "").toLowerCase();
  const highestHrs  = (wk.highest_value / 3600).toFixed(1);

  // Match "monday" → "Mon" etc.
  const isHighest = (label) =>
    highestDay.startsWith(label.toLowerCase());

  return (
    <div className="slide slide-weekday">
      <div className="slide-noise" aria-hidden />
      <div className="weekday-layout">
        <p className="stat-eyebrow" style={{ textAlign: "center" }}>Your peak weekday</p>
        <h2 className="weekday-peak">{wk.highest_day}</h2>
        <p className="weekday-peak-sub">
          avg <strong>{highestHrs}h</strong> per day
        </p>

        <div className="weekday-bars" role="img" aria-label="Average time per weekday bar chart">
          {days.map((d, i) => {
            const pct = (d.value / maxVal) * 100;
            const hi  = isHighest(d.label);
            return (
              <div key={i} className="wbar-col">
                <div className="wbar-track">
                  <div
                    className={`wbar-fill ${hi ? "wbar-hi" : ""}`}
                    style={{ height: started ? `${pct}%` : "0%" }}
                    title={`${(d.value / 3600).toFixed(1)}h avg`}
                  />
                </div>
                <span className={`wbar-label ${hi ? "wbar-label-hi" : ""}`}>{d.label}</span>
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
}

/* 8. Average Per Video ------------------------------------------------------ */
export function AvgVideoSlide({ user }) {
  const started = useStarted();
  const avgSecs = user.activity.average_time_per_vid;

  const personality =
    avgSecs < 10  ? ["Your cooked, get a life", "🫩"]  :
    avgSecs < 15  ? ["You skim at lightning speed.", "⚡"] :
    avgSecs < 20  ? ["You're a selective scroller.", "👀"]  :
                    ["You deep-dive into every video.", "🕳️"];

  return (
    <div className="slide slide-avg">
      <div className="slide-noise" aria-hidden />
      <div className="stat-layout">
        <p className="stat-eyebrow">You gave each video</p>
        <div className="stat-hero">
          <span className="stat-number">
            <CountUp from={0} to={avgSecs} duration={2.2} decimals={1} startCounting={started} />
          </span>
          <span className="stat-unit">seconds</span>
        </div>
        <p className="stat-body">of your attention</p>
        <p className="stat-subtext">{personality[0]}</p>
        <span className="stat-deco" aria-hidden>{personality[1]}</span>
      </div>
    </div>
  );
}

/* 9. Comment Slide --------------------------------------------------------------- */
export function CommentSlide({ user }) {
  const started = useStarted();
  const numComments = user.comments.num_of_comments;

  const personality =
    numComments < 70  ? ["Of the silent type I guess", "🤫"]  :
    numComments < 130  ? ["Not much of a writer", "📩"] :
    numComments < 200  ? ["You know how to express your opinion", "🗣️"]  :
                    ["Do you even stop writing?", "🖊️"];

  return (
    <div className="slide slide-comments">
      <div className="slide-noise" aria-hidden />
      <div className="stat-layout">
        <p className="stat-eyebrow">Last year you wrote</p>
        <div className="stat-hero">
          <span className="stat-number">
            <CountUp from={0} to={numComments} duration={1.8} decimals={0} startCounting={started} />
          </span>
          <span className="stat-unit">comments</span>
        </div>
        <p className="stat-body">and expressed your opinion</p>
        <p className="stat-subtext">{personality[0]}</p>
        <span className="stat-deco" aria-hidden>{personality[1]}</span>
      </div>
    </div>
  );
}

/* 10. Like Slide --------------------------------------------------------------- */
export function LikeSlide({ user }) {
  const started = useStarted();
  const numLikes = user.likes.count_liked_vids;
  const numWatchedVids = user.activity.vids_watched;
  const freqLikes = numWatchedVids / numLikes;

  return (
    <div className="slide slide-likes">
      <div className="slide-noise" aria-hidden />
      <div className="stat-layout">
        <p className="stat-eyebrow">Last year you gave</p>
        <div className="stat-hero">
          <span className="stat-number">
            <CountUp from={0} to={numLikes} duration={2.2} decimals={0} startCounting={started} />
          </span>
          <span className="stat-unit">videos</span>
        </div>
        <p className="stat-body">a like</p>
        <p className="stat-subtext">
          You enjoyed only every <strong><CountUp from={0} to={freqLikes} duration={1.8} decimals={0} startCounting={started} />th</strong> video on average
        </p>
        <span className="stat-deco" aria-hidden>💔</span>
      </div>
    </div>
  );
}

/* 11. Shares Slide --------------------------------------------------------------- */
export function SharesSlide({ user }) {
  const started = useStarted();
  const numShares = user.shares.count_shared_vids;
  const numWatchedVids = user.activity.vids_watched;
  const freqShares = numWatchedVids / numShares;

  return (
    <div className="slide slide-shares">
      <div className="slide-noise" aria-hidden />
      <div className="stat-layout">
        <p className="stat-eyebrow">Last year you shared</p>
        <div className="stat-hero">
          <span className="stat-number">
            <CountUp from={0} to={numShares} duration={2.2} decimals={0} startCounting={started} />
          </span>
          <span className="stat-unit">videos</span>
        </div>
        <p className="stat-body">a like</p>
        <p className="stat-subtext">
          You only deemend every <strong><CountUp from={0} to={freqShares} duration={1.8} decimals={0} startCounting={started} />th</strong> worth sharing on avergae
        </p>
        <span className="stat-deco" aria-hidden>📲</span>
      </div>
    </div>
  );
}

/* 12. Summary --------------------------------------------------------------- */
export function SummarySlide({ user }) {
  const started = useStarted(200);
  const { profile, activity, likes, comments, shares } = user;
  const wk = activity.avergae_time_per_weekday;
  const pfpUrl = `/tiktok_pfp?url=${encodeURIComponent(profile.pfp)}`

const cards = [
  {
    label: "Days on TikTok",
    value: (activity.watch_time_secs / 86400).toFixed(1),
    suffix: "d",
    color: "#7C3AED", 
  },
  {
    label: "Videos watched",
    value: activity.vids_watched.toLocaleString(),
    color: "#06B6D4", 
  },
  {
    label: "Watch sessions",
    value: activity.num_watch_sessions_one_year.toLocaleString(),
    color: "#F97316", 
  },
  {
    label: "Longest binge",
    value: (activity.longest_watch_session?.duration_as_secs / 3600).toFixed(1),
    suffix: "h",
    color: "#EF4444", 
  },
  {
    label: "Most active day",
    value: (activity.most_time_spend_on_tiktok_day.duration_as_secs / 3600).toFixed(1),
    suffix: "h",
    color: "#3B82F6", 
  },
  {
    label: "Avg. per video",
    value: activity.average_time_per_vid.toFixed(1),
    suffix: "s",
    color: "#EAB308", 
  },
  {
    label: "Best weekday",
    value: wk.highest_day,
    color: "#10B981", 
  },
  {
    label: "Avg. Session length",
    value: ((activity.watch_time_secs / activity.num_watch_sessions_one_year) / 60).toFixed(0),
    suffix: "min",
    color: "#60A5FA", 
  },
  {
    label: "Avg. daily watchtime",
    value: (((activity.watch_time_secs / 60. / 60. / 24.) / 365.) * 24.).toFixed(1),
    suffix: "h",
    color: "#C026D3", 
  },
  {
    label: "Comments written",
    value: (comments.num_of_comments),
    suffix: "",
    color: "#F472B6", 
  },
  {
    label: "Videos liked",
    value: (likes.count_liked_vids),
    suffix: "",
    color: "#FB923C", 
  },
  {
    label: "Videos shared",
    value: (shares.count_shared_vids),
    suffix: "",
    color: "#34D399",
  },
];

  return (
    <div className="slide slide-summary">
      <div className="slide-noise" aria-hidden />
      {/* Subtle ambient for summary */}
      <div className="summary-blobs" aria-hidden>
        <div className="sb sb1" /><div className="sb sb2" /><div className="sb sb3" />
      </div>

      <div className="summary-layout">
        {pfpUrl && (
          <div className="summary-pfp-ring">
            <img src={pfpUrl} alt={profile.name} className="summary-pfp" />
          </div>
        )}
        <h2 className="summary-name">@{profile.name}</h2>
        <p className="summary-subtitle">Your TikTok Wrapped</p>

        <div className={`summary-grid ${started ? "sg-show" : ""}`}>
          {cards.map((c, i) => (
            <div
              key={i}
              className="summary-card"
              style={{
                "--card-color": c.color,
                animationDelay: `${i * 90}ms`,
              }}
            >
              <div className="sc-value">
                {c.value}{c.suffix ?? ""}
              </div>
              <div className="sc-label">{c.label}</div>
            </div>
          ))}
        </div>

        <p className="summary-footer">See you next year 🎵</p>
      </div>
    </div>
  );
}

// ─── Slide registry ───────────────────────────────────────────────────────────
export const slides = [
  WelcomeSlide,
  WatchTimeSlide,
  VideosSlide,
  SessionsSlide,
  LongestSessionSlide,
  BestDaySlide,
  WeekdaySlide,
  AvgVideoSlide,
  CommentSlide,
  LikeSlide,
  SharesSlide,
  SummarySlide,
];
