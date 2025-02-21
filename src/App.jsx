import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { getCurrentWindow } from "@tauri-apps/api/window";
import Nav from "./components/Nav";
import Content from "./components/Content";

function App() {
  const [route, setRoute] = useState("day");

  const onRouteChange = async (value) => {
    await invoke('debug_terminal', {variable: "page", value})
    setRoute(value)
  }
  // const wind = getCurrentWindow();

  // for resize print 
  // useEffect(() => {
  //   const sendMessage = async () => {
  //     try {
  //       const w = await wind.innerSize();
  //       await invoke('debug_terminal', {variable: 'size', value: w})
  //     } catch (e) {
  //       await invoke('debug_terminal', {variable: 'error', value: e})
  //       console.log("error", e);
  //     }
  //   }

  //   const unlisten = wind.listen("tauri://resize", sendMessage);
  //   sendMessage();

  //   return () => {
  //     unlisten.then((f) => f());
  //   }
  // }, [])
  return (
    <div className="min-h-screen p-5 bg-zinc-800">
      <Nav onRouteChange={onRouteChange} route={route}/>
      <Content route={route}/>
    </div>
  )
}

export default App;
