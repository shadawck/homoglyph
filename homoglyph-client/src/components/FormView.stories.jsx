import React from 'react';
import { action } from '@storybook/addon-actions';
import { FormButton, SearchBar, InputNumberForOption } from './FormView';
import Form from "./FormView";
import { Spacer } from "@nextui-org/react";

export default {
    /* 👇 The title prop is optional.
    * See https://storybook.js.org/docs/react/configure/overview#configure-story-loading
    * to learn how to generate automatic titles
    */
    title: 'FormView',
    component: FormButton,
    component: SearchBar,
    component: InputNumberForOption,
    component: Form
};



const fakeSendApiRequest = (event) => {
    e.preventDefault()
    console.log("Api request send")
};


export const Button = () => <><Spacer y={2.5} /><FormButton label="compute" onClick={action('clicked')}></FormButton></>
export const Searchbar = () => <><Spacer y={2.5} /><SearchBar placeholder={"Sentence"}></SearchBar></>
export const InputNumber = () => <><Spacer y={2.5} /><InputNumberForOption label="Permutation" state={true}></InputNumberForOption></>
export const _Form = () => <><Spacer y={2.5} /><Form onSubmit={null} /></>