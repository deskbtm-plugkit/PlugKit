// import { invoke } from '@tauri-apps/api/tauri';
import './App.css';
import React from 'react';
import { invoke } from '@tauri-apps/api';
import { useRef, useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import { appWindow, WebviewWindow } from '@tauri-apps/api/window';
import { homeDir, resolve } from '@tauri-apps/api/path';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { Route, BrowserRouter, Routes, useNavigate } from 'react-router-dom';
import { isPermissionGranted } from '@tauri-apps/api/notification';

const children: React.ReactNode[] = [];

const unlisten = listen('demo1', (event) => {
  console.log('============================', event);
  // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
  // event.payload is the payload object
});

appWindow.listen<string>('state-changed', (event) => {
  console.log(`Got error: `, event);
});

const handleChange = (value: string) => {
  console.log(`selected ${value}`);
};

function Page1() {
  const [count, setCount] = useState(0);
  const [url, setUrl] = useState<string | null>(null);
  const ref = useRef<HTMLDivElement>(null);

  const navigate = useNavigate();
  // appWindow.show();

  return (
    <div className="App">
      {/* <div data-tauri-drag-region className="titlebar">
        <div className="titlebar-button" id="titlebar-minimize">
          <img
            src="https://api.iconify.design/mdi:window-minimize.svg"
            alt="minimize"
          />
        </div>
        <div className="titlebar-button" id="titlebar-maximize">
          <img
            src="https://api.iconify.design/mdi:window-maximize.svg"
            alt="maximize"
          />
        </div>
        <div className="titlebar-button" id="titlebar-close">
          <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
        </div>
      </div> */}
      {/* <button
        onClick={async () => {
          const r = await invoke('my_custom_command');
          console.log(r);
        }}
      >
        click
      </button> */}
      <div ref={ref}></div>

      <button
        onClick={() => {
          navigate('/page2');
        }}
      >
        Demo
      </button>
      {/* <iframe src="" frameBorder="0"></iframe> */}
      <button
        onClick={async () => {
          const permissionGranted = await isPermissionGranted();
          console.log(permissionGranted);

          // sendNotification('Tauri is awesome!');
          // sendNotification({
          //   title: 'TAURI',
          //   body: 'Tauri is awesome!',
          //   icon: '/vite.svg',
          // });
          // console.log('=======');
          const w = new WebviewWindow('Wallpaper', {
            url: '/built-in/wallpaper/src/index.html',
          });

          // await invoke('create_demo_window');
        }}
      >
        new Window
      </button>
      <button
        onClick={() => {
          // "geolocation" | "notifications" | "persistent-storage" | "push" | "screen-wake-lock" | "xr-spatial-tracking";
          navigator.permissions.query({ name: 'geolocation' }).then((e) => {
            console.log(e);
          });
          navigator.permissions.query({ name: 'notifications' }).then((e) => {
            console.log(e);
          });
          navigator.permissions
            .query({ name: 'persistent-storage' })
            .then((e) => {
              console.log(e);
            });
          navigator.permissions.query({ name: 'push' }).then((e) => {
            console.log(e);
          });

          navigator.permissions
            .query({ name: 'screen-wake-lock' })
            .then((e) => {
              console.log(e);
            });

          navigator.clipboard.read().then((e) => {
            console.log(e);
          });
          // appWindow.hide();
        }}
      >
        Clipboard
      </button>
      <button onClick={() => {}}>Hide</button>
      <button
        onClick={async () => {
          // import('http://osd.deskbtm.com/case.js' as any).then((e) => {
          //   console.log(e);
          // });

          // const webview = new WebviewWindow('theUniqueLabel', {
          //   url: 'http://osd.deskbtm.com/case.html',
          // });

          const home = await homeDir();
          const a = await resolve(
            home,
            'AbyssProject/abyss/app/public/case.html',
          );

          const htmlUri = convertFileSrc(a);

          setUrl(htmlUri);
          console.log(htmlUri);
          // await appWindow.setFullscreen(true);
          // await appWindow.maximize();

          // await appWindow.setSize()

          // setTimeout(async () => {
          //   const r = await invoke('my_custom_command');
          // }, 4000);

          // console.log(ref.current);
          // ref.current!.innerHTML =
          //   '<iframe id="inlineFrameExample" title="Inline Frame Example" width="300" height="200" src="http://www.baidu.com"></iframe>';
          // console.log(r);
        }}
      >
        Plugin
      </button>

      <button
        onClick={async () => {
          const r = await invoke('cmd1');
        }}
      >
        cmd1
      </button>

      <button
        onClick={() => {
          invoke('exec_planet');
        }}
      >
        exec_planet
      </button>

      <img src="/vite.svg" />

      {url && (
        <iframe
          id="inlineFrameExample"
          title="Inline Frame Example"
          width="300"
          height="200"
          style={{ border: 'none' }}
          src={url!}
        ></iframe>
      )}

      <div
        className="box"
        style={{
          width: 300,
          height: 300,
          background: 'pink',
        }}
      ></div>
      {Array.from({ length: 100 }).map((v, i) => {
        return <br key={i} />;
      })}
    </div>
  );
}

function Page2() {
  return <div>Page2</div>;
}

function App() {
  return (
    <BrowserRouter basename="/app/src">
      <Routes>
        <Route path="/">
          <Route index element={<Page1 />} />
          <Route path="/page2" element={<Page2 />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
