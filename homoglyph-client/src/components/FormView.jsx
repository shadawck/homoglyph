import { Button, Card, Input, Spacer, Container, Navbar, Link, Textarea, Row, Col, Grid, Text } from "@nextui-org/react";
import { useState, Component } from "react";

class FormInputOption extends Component {
    constructor(props) {
        super(props)
        this.handleChange = this.handleChange.bind(this);
    }

    handleChange(event) {
        console.log(event.target.value)
        //this.setState({ number: event.target.value });
        this.props.onNumberChange(event.target.value)
    }

    render() {
        const name = this.props.name;
        const label = this.props.label;
        const number = this.props.number;

        return (
            <div>
                <Spacer y={0.5} />
                <Input
                    value={number}
                    onChange={this.handleChange}
                    name={name}
                    id={name}
                    labelPlaceholder={label}
                    rounded
                    bordered
                    color="primary"
                    type="number"
                    size="sm"
                    width="10em"
                    status="primary"
                    aria-label={name} />
                <Spacer y={0.5} />
            </div>
        )

    }
}

class SearchBar extends Component {
    constructor(props) {
        super(props)
        this.handleChange = this.handleChange.bind(this);
    }

    handleChange(event) {
        this.props.onSearchChange(event.target.value);
    }

    render() {
        const sentence = this.props.sentence;
        const name = this.props.name;

        return (
            <div>
                <Spacer y={0.5} />
                <Input
                    css={{ minWidth: "500px" }}
                    onChange={this.handleChange}
                    value={sentence}
                    name={name}
                    id={name}
                    aria-label="SearchBar"
                    rounded
                    clearable
                    bordered
                    labelPlaceholder={this.props.placeholder}
                    color="primary"
                    type="search" />
            </div>
        )

    }
};

export const FormButton = ({ label }) => {
    return (
        <Button
            type="submit"
            aria-label="Submit"
        >{label}
        </Button >
    );
};


class Form extends Component {
    constructor(props) {
        super(props)
        this.handlePermutationChange = this.handlePermutationChange.bind(this);
        this.handleConfusableChange = this.handleConfusableChange.bind(this);
        this.handleSearchChange = this.handleSearchChange.bind(this);
        this.handleSubmit = this.handleSubmit.bind(this);

        this.state = {
            sentence: "",
            permutation: "",
            confusable: "",
            homoglyphsResponse: ""
        }
    }

    handleConfusableChange(number) {
        this.setState({ confusable: number })
    }

    handlePermutationChange(number) {
        this.setState({ permutation: number })
    }

    handleSearchChange(sentence) {
        this.setState({ sentence })
    }

    async handleSubmit(event) {
        event.preventDefault()
        const requestOptions = {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Access-Control-Allow-Origin': '*'

            },
            body: JSON.stringify({
                sentence: this.state.sentence,
                permutation: parseInt(this.state.permutation),
                confusable: parseInt(this.state.confusable)
            }),
        };

        console.log(requestOptions.body)

        const response = await fetch('/api/homoglyphs', requestOptions);

        if (!response.ok) {
            throw new Error('Homoglyphs coud not be computed!')
        } else {
            response.json()
                .then(res => this.setState({ homoglyphsResponse: JSON.stringify(res) }))
                .catch((e) => console.log(e.message));
        }
    }


    render() {
        const sentence = this.state.sentence;
        const permutation = this.state.permutation;
        const confusable = this.state.confusable;
        const homoglyphs = this.state.homoglyphsResponse;

        return (

            <Grid.Container gap={2} justify="center">
                <Grid xs={12} md={6} justify="center" >
                    <Container>
                        <Card css={{ minWidth: "200px", minHeight: "200px" }} >
                            <Card.Header>
                                <Text weight="bold" h2>Input</Text>
                            </Card.Header>
                            <Card.Divider />
                            <Card.Body css={{ py: "$10" }}>
                                < form onSubmit={this.handleSubmit} >
                                    <SearchBar placeholder={"sentence"} name="sentence" onSearchChange={this.handleSearchChange} sentence={sentence} />
                                    <Spacer y={0.5} />
                                    <Grid.Container gap={2}>
                                        <Grid>
                                            <FormInputOption label="Permutation" name="permutation" onNumberChange={this.handlePermutationChange} number={permutation} />
                                        </Grid>
                                        <Grid>
                                            <FormInputOption label="Confusable" name="confusable" onNumberChange={this.handleConfusableChange} number={confusable} />
                                        </Grid>
                                    </Grid.Container>
                                    <FormButton label={"Compute"} />
                                </form>
                            </Card.Body>
                        </Card>
                    </Container>
                </Grid>


                <DisplayResultArea homoglyphs={homoglyphs} />


            </Grid.Container >
        );
    }
}

class DisplayResultArea extends Component {
    constructor(props) {
        super(props)
    }

    render() {
        const status = this.props.status;
        const homoglyphs = this.props.homoglyphs;

        return (

            <Grid xs={12} md={6}>
                <Container >
                    <Card css={{ minWidth: "200px", minHeight: "500px" }}>
                        <Card.Header>
                            <Text weight="bold" h2>Display Results</Text>
                        </Card.Header>
                        <Card.Divider />
                        <Card.Body css={{ py: "$10" }}>
                            <Grid.Container>
                                <Grid>
                                    <Text>{homoglyphs}</Text>
                                    {/* <Textarea
                                        fullWidth={true}
                                        value={homoglyphs}
                                        label="Homoglyphs Results"
                                        bordered
                                        color={status}
                                        placeholder="Write a sentence, choose options and click compute to see result"
        /> */}
                                </Grid>
                            </Grid.Container>
                        </Card.Body>
                    </Card>
                </Container>
            </Grid >

        );
    }

}

export { FormInputOption, Form };