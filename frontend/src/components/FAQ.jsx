import { useState, useRef } from "react";
import "./faq.css";

const FAQS = [
  {
    q: "How can I get my TikTok data?",
    a: <>
      Go to <a href="https://www.tiktok.com/setting/download-your-data" target="_blank" rel="noreferrer" className="faq-link">tiktok.com/setting/download-your-data</a> or open TikTok → Profile → Settings and privacy → Account → Download your data.
      <br /><br />
      Make sure to select <strong>JSON – Machine-readable file</strong> as the format, otherwise this won't be able to read your data. After requesting, TikTok may take <strong>a few days</strong> to prepare your export.
    </>,
  },
  {
    q: "Which file should I upload?",
    a: <>
      Once TikTok emails you that your data is ready, download it and unzip the archive. Look for a file called <strong>user_data_tiktok.json</strong> (or similar), that's the one to upload here.",
    </>,
  },
  {
    q: "Is this safe? Is my data private?",
    a: <>
      Yes. Your file is <strong>only parsed on your browser</strong> and exists only on your device.
      Nothing is ever uploaded to a server, <strong>your data never leaves your device.</strong>{" "}
      This whole website was only made because I didn't want to upload my data to the more well known TikTok Wrapped.
    </>,
  },
  {
    q: "What counts as a 'watch session'?",
    a: "A watch session is a continuous block of TikTok usage. If you close the app or stop watching for a while, a new session starts next time you open it.",
  },
  {
    q: "What counts as a 'view'?",
    a: "Any video that appears in your feed and registers in TikTok's activity log counts as a view, even if you only saw a few seconds of it.",
  },
  {
    q: "Does my Wrapped cover my full TikTok history?",
    a: "It depends on how much data TikTok included in your export. Most exports go back at least a year, but older activity may not be present.",
  },
  {
    q: "How long do I have to wait for my TikTok data?",
    a: "TikTok usually takes between a few hours and a few days to prepare your export. You'll get an in-app notification and an email when it's ready.",
  },
];

function FAQItem({ q, a }) {
  const [open, setOpen] = useState(false);
  const contentRef = useRef(null);

  const toggle = () => setOpen((o) => !o);

  return (
    <div className={`faq-item ${open ? "faq-open" : ""}`}>
      <button
        className="faq-question"
        onClick={toggle}
        aria-expanded={open}
      >
        <span>{q}</span>
        <span className="faq-chevron" aria-hidden>{open ? "∧" : "v"}</span>
      </button>
      <div
        className="faq-answer-wrap"
        style={{
          maxHeight: open ? contentRef.current?.scrollHeight + "px" : "0px",
        }}
      >
        <p className="faq-answer" ref={contentRef}>{a}</p>
      </div>
    </div>
  );
}

export default function FAQ() {
  return (
    <section className="faq-section">
      <h2 className="faq-title">Frequently Asked Questions</h2>
      <div className="faq-list">
        {FAQS.map((item, i) => (
          <FAQItem key={i} {...item} />
        ))}
      </div>
    </section>
  );
}
