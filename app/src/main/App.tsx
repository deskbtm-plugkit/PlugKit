// import { invoke } from '@tauri-apps/api/tauri';
import { useRef, useState } from 'react';
import { Select } from 'antd';
import React from 'react';
import 'antd';
import 'antd/dist/antd.css';
import { appWindow } from '@tauri-apps/api/window';
import { appDir, join, homeDir, resolve } from '@tauri-apps/api/path';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { invoke } from '@tauri-apps/api';
import { WebviewWindow } from '@tauri-apps/api/window';
import './App.css';

const { Option } = Select;

const children: React.ReactNode[] = [];
for (let i = 10; i < 36; i++) {
  children.push(<Option key={i.toString(36) + i}>{i.toString(36) + i}</Option>);
}

appWindow.listen<string>('state-changed', (event) => {
  console.log(`Got error: `, event);
});

const handleChange = (value: string) => {
  console.log(`selected ${value}`);
};

function App() {
  const [count, setCount] = useState(0);
  const [url, setUrl] = useState<string | null>(null);
  const ref = useRef<HTMLDivElement>(null);

  return (
    <div className="App">
      {/* <button
        onClick={async () => {
          const r = await invoke('my_custom_command');
          console.log(r);
        }}
      >
        click
      </button> */}
      <div ref={ref}></div>
      {/* <iframe src="" frameBorder="0"></iframe> */}
      <button
        onClick={() => {
          console.log('=======');
          const w = new WebviewWindow('Setting', {
            url: 'http://localhost:5173/setting/index.html',
          });
          console.log(w);
        }}
      >
        new Window
      </button>
      <button
        onClick={async () => {
          // import('http://osd.deskbtm.com/case.js' as any).then((e) => {
          //   console.log(e);
          // });

          await appWindow.emit('state-changed', {
            loggedIn: true,
            token: 'authToken',
          });

          // const webview = new WebviewWindow('theUniqueLabel', {
          //   url: 'http://osd.deskbtm.com/case.html',
          // });

          const home = await homeDir();
          const a = await resolve(
            home,
            'AbyssProject/abyss/app/public/case.html',
          );
          console.log(a);

          const htmlUri = convertFileSrc(a);

          setUrl(htmlUri);
          console.log(htmlUri);

          // console.log(ref.current);
          // ref.current!.innerHTML =
          //   '<iframe id="inlineFrameExample" title="Inline Frame Example" width="300" height="200" src="http://www.baidu.com"></iframe>';
          // const r = await invoke('my_custom_command');
          // console.log(r);
        }}
      >
        Plugin
      </button>

      <img src="/vite.svg" />

      <iframe
        id="inlineFrameExample"
        title="Inline Frame Example"
        width="300"
        height="200"
        style={{ border: 'none' }}
        src={url!}
      ></iframe>

      <div
        className="box"
        style={{
          width: 300,
          height: 300,
          background: 'pink',
        }}
      ></div>
    </div>
  );
}

export default App;
