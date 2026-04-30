import { createTheme, MantineColorsTuple } from "@mantine/core";

const primaryColor: MantineColorsTuple = [
  "#e8f5e9",
  "#c8e6c9",
  "#a5d6a7",
  "#81c784",
  "#66bb6a",
  "#4caf50",
  "#43a047",
  "#388e3c",
  "#2e7d32",
  "#1b5e20",
];

export const theme = createTheme({
  primaryColor: "erp-green",
  colors: {
    "erp-green": primaryColor,
  },
  fontFamily: "Inter, -apple-system, BlinkMacSystemFont, sans-serif",
  defaultRadius: "md",
  components: {
    Button: {
      defaultProps: { radius: "md" },
    },
    TextInput: {
      defaultProps: { radius: "md" },
    },
    Select: {
      defaultProps: { radius: "md" },
    },
    Card: {
      defaultProps: { radius: "md", shadow: "sm" },
    },
    Paper: {
      defaultProps: { radius: "md", shadow: "sm" },
    },
  },
});
