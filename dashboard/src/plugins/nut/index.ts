import { IRoute } from "..";

const routes: IRoute[] = [
    {
        component: () => import("./users/SignIn"),
        path: "/users/sign-in",
    }, {
        component: () => import("./users/SignUp"),
        path: "/users/sign-up",
    },
]

export default routes;