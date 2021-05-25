# Parser
There are three mains parts to the parser
# Raw
This reads the raw file and converts the text to a list of `Statement` structs that can then be converted into a `Function` using the `Function::from_raw` API.
# Process
This transforms all the statements that aren't function declarations into a big function and calls that function.
# Function
This creates `Function` structs based on the list of Statements that are considered `function scope` - `{` `..` `}`