import { useNavigate } from "react-router-dom";
import "./App.css";
import MagicBento from "./block/Components/MagicBento/MagicBento";

function App() {
  const navigate = useNavigate();

  const handleCardClick = () => {
    navigate("/login");
  };

  return (
    <div className="" onClick={handleCardClick}>
      <MagicBento
        textAutoHide={true}
        enableStars={true}
        enableSpotlight={true}
        enableBorderGlow={true}
        enableTilt={true}
        enableMagnetism={true}
        clickEffect={true}
        spotlightRadius={300}
        particleCount={12}
        glowColor="132, 0, 255"
      />
    </div>
  );
}

export default App;
