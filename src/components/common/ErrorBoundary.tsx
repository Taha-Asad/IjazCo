import { Component, ErrorInfo, ReactNode } from "react";
import { Stack, Title, Text, Button, ThemeIcon, Paper } from "@mantine/core";
import { IconAlertTriangle } from "@tabler/icons-react";

interface Props {
  children: ReactNode;
  fallback?: ReactNode;
}

interface State {
  hasError: boolean;
  error: Error | null;
}

export class ErrorBoundary extends Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = { hasError: false, error: null };
  }

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, info: ErrorInfo) {
    console.error("ErrorBoundary caught:", error, info);
  }

  handleReset = () => {
    this.setState({ hasError: false, error: null });
  };

  render() {
    if (this.state.hasError) {
      if (this.props.fallback) return this.props.fallback;

      return (
        <Paper withBorder p="xl" radius="md" m="md">
          <Stack align="center" gap="md">
            <ThemeIcon size={64} radius="xl" color="red" variant="light">
              <IconAlertTriangle size={32} />
            </ThemeIcon>
            <div style={{ textAlign: "center" }}>
              <Title order={3} mb="xs">
                Something went wrong
              </Title>
              <Text c="dimmed" size="sm" mb="md">
                {this.state.error?.message || "An unexpected error occurred."}
              </Text>
            </div>
            <Button onClick={this.handleReset} variant="light">
              Try Again
            </Button>
          </Stack>
        </Paper>
      );
    }

    return this.props.children;
  }
}
