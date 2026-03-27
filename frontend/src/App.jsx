import { useState } from "react";
import { parse_data } from "tiktok_wrapped_v2";
import CountUp from './utils/CountUp';

function App() {
  const [user, setUser] = useState(null);

  const handleFile = async (event) => {
    const file = event.target.files[0];
    if (!file) return;

    const text = await file.text();   // JS reads file
    const result = parse_data(text);  // Rust parses JSON

    setUser(result);
  };

  return (
    <div style={{ padding: "20px" }}>
      <h1>Upload TikTok JSON 👇</h1>

      <input type="file" onChange={handleFile} />

      {user && (
        <div style={{ marginTop: "20px" }}>
          <h2>Results:</h2>

          <img
            src={user.profile.pfp}
            alt="example"
            style={{ width: "100px" }}
          />
          <p>Hello {user.profile?.name}!</p>
          <p>
            Watch Time:{" "}
            <CountUp
              from={0}
              to={parseInt(user.activity?.watch_time_secs / 60.0 / 60.0 / 24.0)}
              separator=","
              direction="up"
              duration={0.25}
              className="count-up-text"
              startCounting
            />
            {" "}Days
          </p>
          <p>
            Videos watched:{" "}
            <CountUp
              from={0}
              to={user.activity.vids_watched}
              separator=","
              direction="up"
              duration={0.25}
              className="count-up-text"
              startCounting
            />
          </p>
        </div>
      )}
    </div>
  );
}

export default App;