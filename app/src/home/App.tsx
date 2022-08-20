// import { invoke } from '@tauri-apps/api/tauri';
import { useRef, useState } from 'react';
import { Select } from 'antd';
import React from 'react';
import 'antd';
import 'antd/dist/antd.css';
import './App.css';
import { invoke } from '@tauri-apps/api';

const { Option } = Select;

const children: React.ReactNode[] = [];
for (let i = 10; i < 36; i++) {
  children.push(<Option key={i.toString(36) + i}>{i.toString(36) + i}</Option>);
}

const handleChange = (value: string) => {
  console.log(`selected ${value}`);
};

function App() {
  const [count, setCount] = useState(0);
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
        onClick={async () => {
          import('./a.js' as any).then((e) => {
            console.log(e);
          });
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
