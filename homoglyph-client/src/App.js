import logo from './logo.svg';
import './App.css';
import { Form } from './components/FormView';
import { Layout } from "./components/Layout";
import { Button, Card, Input, Spacer, Container, Navbar, Link, Textarea, Row, Col, Grid, Text } from "@nextui-org/react";

function App() {

  return (
    <div className="">
      <Layout>
        <Navbar isBordered variant="floating">
          <Navbar.Brand>

            <Text b color="inherit" hideIn="xs">
              Homoglyphs Calculator
            </Text>
          </Navbar.Brand>
          <Navbar.Content hideIn="" variant="underline">
            {/*<Navbar.Link href="#">Github</Navbar.Link>*/}
          </Navbar.Content>
          <Navbar.Content>
            <Navbar.Link color="inherit" href="https://github.com/shadawck/homoglyph">
              About
            </Navbar.Link>
            <Navbar.Item>
              <Button auto flat as={Link} href="#">
                Github
              </Button>
            </Navbar.Item>
          </Navbar.Content>
        </Navbar>
      </Layout>
    </div>
  );
}

export default App;