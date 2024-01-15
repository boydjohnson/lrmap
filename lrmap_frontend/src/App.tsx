import { useState, useEffect } from 'react';
import './App.css';

function App() {
  const [svg, setSvg] = useState<string>('');

  useEffect(() => {
    const fetchSvg = async () => {
      try {
        const response = await fetch('https://svc.lrmap.com');
        const svgData = await response.text();
        setSvg(svgData);
      } catch (error) {
        console.error('Error fetching SVG:', error);
      }
    };

    fetchSvg();

    const interval = setInterval(fetchSvg, 15000);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="App">
      <div dangerouslySetInnerHTML={{ __html: svg }} />
      <div><p>Realtime map of light rail train locations in the Twin Cities</p></div>
    </div>
  );
}

export default App;
