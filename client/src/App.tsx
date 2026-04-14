import { Route, Router } from "@solidjs/router";
import { Nav } from "./components/Nav";
import { HomeRoute } from "./routes/Home";
import { ViewList } from "./routes/ViewList";

function App() {
  return (
    <>
      <Nav />
      <Router>
        <Route path="/" component={HomeRoute} />
        <Route path="/list/:id" component={ViewList} />
      </Router>
    </>
  );
}

export default App;
