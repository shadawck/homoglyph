import { Button, Input, Checkbox, Spacer, Container } from "@nextui-org/react";
import { useState } from "react";


/// Add helper : https://nextui.org/docs/components/input#helper-text

export const FormButton = ({ label, onClick }) => {
    return (
        <Button
            type="submit"
            aria-label="Submit"
            onClick={onClick}>
            {label}
        </Button>
    );
};
export const SearchBar = ({ placeholder }) => {
    return (
        <Input
            aria-label="SearchBar"
            rounded
            clearable
            bordered
            labelPlaceholder={placeholder}
            color="primary"
            type="search" />
    )
}

export const InputNumberForOption = ({ label, state, onChange, value, name }) => {
    const [selected, setSelected] = useState(state);

    return (
        <Container gap={0} css={{ d: 'flex', flexWrap: 'nowrap' }}>
            <Checkbox
                isRounded={true}
                isSelected={selected}
                onChange={setSelected}
                size="md"
                color="primary"
            >
            </Checkbox>
            <Spacer x={1} />
            <Input
                onChange={onChange}
                value={value}
                name={name}
                labelPlaceholder={label}
                disabled={!selected}
                rounded
                bordered
                color="primary"
                type="number"
                size="sm"
                width="10em"
                status="primary"
                aria-label={name} />
        </Container>
    )
}

export const Form = () => {
    const [formData, setFormData] = useState({
        sentence: "",
        permutation: "",
        confusable: ""
    });

    const handleChange = (event) => {
        setFormData({ ...formData, [event.target.name]: event.target.value });
    };

    const sendApiRequest = (e) => {
        e.preventDefault()
        const requestOptions = {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'

            },
            body: JSON.stringify(formData)
        };

        fetch('http://127.0.0.1:8000/api/homoglyphs', requestOptions)
            .then(response => response.json())
            .then(data => console.log(data));
    }

    return (
        < form onSubmit={sendApiRequest} >
            <label htmlFor="title">Title</label><br />
            <Input placeholder={"Sentence"} onChange={handleChange} value={formData.sentence} type="text" name="sentence" id="sentence" /><br /><br />

            <label htmlFor="permutation">permutation</label><br />
            <InputNumberForOption state={false} label={"Permutation"} onChange={handleChange} value={formData.permutation} name="permutation" id="permutation" />
            <br />

            <label htmlFor="confusable">confusable</label><br />
            <InputNumberForOption state={false} label={"Confusable"} onChange={handleChange} value={formData.confusable} name="confusable" id="confusable" />
            <br />

            <FormButton label={"Compute"} onClick={null} />
        </form >
    )
}