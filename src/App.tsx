import { Header } from "./components/Header";
import ListFilesToAdd from "./components/ListFilesToAdd";

function App() {
  return (
    <div className="flex flex-col items-center justify-center">
      <Header />
      <ListFilesToAdd />
    </div>
  );
}

export default App;
