import "./App.css";
import AppBody from "./AppBody.tsx";

function App() {
  return (
    <div className="app">
      <div className="title-bar">
        <h1>Meine Tauri App</h1>
      </div>
      <div className="content">
        <AppBody />
      </div>
    </div>
  );
}

export default App;
