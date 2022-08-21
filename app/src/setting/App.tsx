import { useRef, useState } from 'react';
import { Select } from 'antd';
import React from 'react';

const { Option } = Select;

function App() {
  const [count, setCount] = useState(0);
  const ref = useRef<HTMLDivElement>(null);

  return (
    <div className="App">
      <div ref={ref}></div>
      <button onClick={async () => {}}>Plugin</button>

      <div
        className="box"
        style={{
          width: 300,
          height: 300,
          background: 'green',
        }}
      ></div>
    </div>
  );
}

export default App;
