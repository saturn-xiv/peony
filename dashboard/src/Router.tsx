import React, { Suspense, lazy } from "react";
import { BrowserRouter as Router, Route, Switch } from "react-router-dom";

import routes from "./plugins";

interface IProps {
  basename: string;
}

const Component = ({ basename }: IProps) => {
  return (
    <Router basename={basename}>
      <Suspense fallback={<div>Loading...</div>}>
        <Switch>
          {routes.map((it) => (
            <Route
              key={it.path}
              path={it.path}
              exact={true}
              component={lazy(it.component)}
            />
          ))}
          <Route
            path="/"
            exact={true}
            component={lazy(() => import("./plugins/nut/Home"))}
          />
          <Route component={lazy(() => import("./plugins/nut/NotFound"))} />
        </Switch>
      </Suspense>
    </Router>
  );
};



export default Component;
