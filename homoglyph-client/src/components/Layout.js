import { Form } from "./FormView"
import { Box } from "./Box";

export const Layout = ({ children }) => (
    <Box
        css={{
            //Custom css
        }}
    >
        {children}
        <Form />
    </Box>
);