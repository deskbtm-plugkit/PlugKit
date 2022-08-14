// import { invoke } from '@tauri-apps/api/tauri';
import { useState } from 'react';
import { Select } from 'antd';
import React from 'react';
import 'antd';
import 'antd/dist/antd.css';
import './App.css';

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

  return (
    <div className="App">
      <Select
        mode="tags"
        style={{ width: '100%' }}
        placeholder="Tags Mode"
        onChange={handleChange}
      >
        {children}
      </Select>
    </div>
  );
}

export default App;
