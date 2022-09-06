import React from 'react';
import { MainUI } from './main';
import { Spacer } from "@nextui-org/react";

export default {
    title: 'MainUI',
    component: MainUI
};


export const Main = () => <><Spacer y={2.5} /><MainUI /></>
