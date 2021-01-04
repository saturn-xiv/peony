import React from "react";
import { useIntl } from "react-intl";
import { useHistory } from "react-router";

interface IProps {
    children: React.ReactNode;
    title: string;
}


const Component = ({ title, children }: IProps) => {
    const intl = useIntl();
    const history = useHistory();

    return (
        <div>
            <h1>dashboard</h1>
            {children}
        </div>
    );
};

export default Component;