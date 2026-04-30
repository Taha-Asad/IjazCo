import {
  TextInput,
  Button,
  Paper,
  Title,
  Text,
  Container,
  Stack,
  Anchor,
  Alert,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { useState } from "react";
import { Link } from "react-router-dom";
import { IconCheck } from "@tabler/icons-react";
import { authApi } from "../../api/auth";

export function ForgotPasswordPage() {
  const [loading, setLoading] = useState(false);
  const [sent, setSent] = useState(false);

  const form = useForm({
    initialValues: { email: "" },
    validate: {
      email: (v) => (!/^\S+@\S+$/.test(v) ? "Valid email required" : null),
    },
  });

  const handleSubmit = async (values: typeof form.values) => {
    setLoading(true);
    try {
      await authApi.requestPasswordReset(values.email);
      setSent(true);
    } catch {
      setSent(true); // Always show success (security)
    } finally {
      setLoading(false);
    }
  };

  return (
    <Container size={440} my={80}>
      <Title ta="center" fw={900}>
        Forgot Password
      </Title>
      <Text c="dimmed" size="sm" ta="center" mt={5}>
        Enter your email to receive reset instructions
      </Text>
      <Paper withBorder shadow="md" p={30} mt={30} radius="md">
        {sent ? (
          <Stack align="center">
            <Alert icon={<IconCheck />} color="green" radius="md" w="100%">
              If that email exists, you'll receive reset instructions shortly.
            </Alert>
            <Anchor component={Link} to="/login">
              Back to Login
            </Anchor>
          </Stack>
        ) : (
          <form onSubmit={form.onSubmit(handleSubmit)}>
            <Stack>
              <TextInput
                label="Email Address"
                placeholder="your@email.com"
                required
                {...form.getInputProps("email")}
              />
              <Button type="submit" fullWidth loading={loading}>
                Send Reset Link
              </Button>
              <Text ta="center" size="sm">
                <Anchor component={Link} to="/login">
                  Back to Login
                </Anchor>
              </Text>
            </Stack>
          </form>
        )}
      </Paper>
    </Container>
  );
}
