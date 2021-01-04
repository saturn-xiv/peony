import React, { useState } from "react";
import { useDispatch } from "react-redux";
import { useIntl } from "react-intl";
import { useHistory } from "react-router";


import Layout from "../../../layouts/application";



export default () => {
    const history = useHistory();
    const intl = useIntl();
    const dispatch = useDispatch();
    return (<Layout title="sign up">sign up</Layout>)
}