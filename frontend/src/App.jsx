import { useState } from "react";
import { parse_data } from "tiktok_wrapped_v2";
import Slideshow from "./components/Slideshow";
import "./App.css";

import FAQ from "./components/FAQ";
import JSZip from "jszip";

export default function App() {
  const [user, setUser]       = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError]     = useState(null);
  const [dragOver, setDragOver] = useState(false);

  const processFile = async (file) => {
    if (!file) return;

    const isJson = file.name.endsWith(".json");
    const isZip =  file.name.endsWith(".zip");
    if (!isJson && !isZip) {
      setError("That doesn't look like a JSON or ZIP file. Try again.");
      return;
    }
    setLoading(true);
    setError(null);

    try {
      let text;
      if (isZip) {
        const zip = await JSZip.loadAsync(file);
        // TikTok nennt die Datei "user_data.json" im ZIP
        const jsonFile = zip.file("user_data_tiktok.json");
        if (!jsonFile) {
          setError("Couldn't find user_data_tiktok.json in the ZIP. Make sure it's your TikTok export.");
          return;
        }
        text = await jsonFile.async("string");
      } else {
        text = await file.text();
      }
      
      const result = parse_data(text);
      setUser(result);
    } catch (e) {
      console.error(e);
      setError("Couldn't parse your file. Make sure it's your TikTok data export JSON.");
    } finally {
      setLoading(false);
    }
  };

  const handleFile = (e)  => processFile(e.target.files[0]);
  const handleDrop = (e)  => {
    e.preventDefault();
    setDragOver(false);
    processFile(e.dataTransfer.files[0]);
  };

  if (user) return <Slideshow user={user} onReset={() => setUser(null)} />;

  return (
    <div
      className={`upload-screen ${dragOver ? "drag-over" : ""}`}
      onDragOver={(e) => { e.preventDefault(); setDragOver(true); }}
      onDragLeave={() => setDragOver(false)}
      onDrop={handleDrop}
    >
      {/* Ambient blobs */}
      <div className="upload-blobs" aria-hidden>
        <div className="ublob ublob-pink" />
        <div className="ublob ublob-cyan" />
        <div className="ublob ublob-pink2" />
      </div>

      <div className="upload-inner">
        {/* TikTok 3-layer glitch logo */}
        <div className="tt-logo" aria-hidden>
          <span className="ttl-cyan">TT</span>
          <span className="ttl-pink">TT</span>
          <span className="ttl-main">TT</span>
        </div>

        <h1 className="upload-title">
          TikTok<br />Wrapped
        </h1>

        <p className="upload-sub">
          {dragOver
            ? "Drop it! 🎯"
            : "Your year in scroll. Upload your TikTok data export to begin."}
        </p>

        <label className="upload-btn">
          <input type="file" accept=".json,.zip" onChange={handleFile} hidden />
          {loading
            ? <span className="loading-dots">Parsing<span>.</span><span>.</span><span>.</span></span>
            : "Choose File"}
        </label>

        {error && <p className="upload-error">{error}</p>}

        <p className="upload-hint">
          TikTok → Settings → Account → Download your Data → Request JSON Format → Choose all boxes
        </p>

        <div style={{ marginTop: "150px" }}> {/* für später: content von dem faq in FAQ.jsx ändern */}
          <FAQ />
        </div>
      </div>
    </div>
  );
}
