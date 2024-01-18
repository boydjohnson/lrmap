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
      <div className="map-container">
        <nav>
          <div id="tagline"><div>lrmap</div>Realtime Twin Cities Light Rail Info</div>


          <div id="lastUpdate">Updated a few seconds ago</div>
        </nav>

        <div id="map" dangerouslySetInnerHTML={{ __html: svg }} />
        <div><p>Realtime map of light rail train locations in the Twin Cities</p></div>
      </div>
    </div>
  );
}

export default App;
