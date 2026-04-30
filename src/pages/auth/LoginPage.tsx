import {
  TextInput,
  PasswordInput,
  Button,
  Paper,
  Title,
  Text,
  Container,
  Stack,
  Anchor,
  Alert,
  Group,
  Divider,
  ThemeIcon,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { z } from "zod";
import { useState } from "react";
import { useNavigate, Link } from "react-router-dom";
import { notifications } from "@mantine/notifications";
import { IconAlertCircle, IconLock } from "@tabler/icons-react";
import { authApi } from "../../api/auth";
import { useAuthStore } from "../../store/authStore";
import { zodResolver } from "mantine-form-zod-resolver";

const schema = z.object({
  username: z.string().min(1, "Username is required"),
  password: z.string().min(1, "Password is required"),
});

export function LoginPage() {
  const navigate = useNavigate();
  const { setUser, setTokens } = useAuthStore();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const form = useForm({
    validate: zodResolver(schema),
    initialValues: { username: "", password: "" },
  });

  const handleSubmit = async (values: typeof form.values) => {
    setLoading(true);
    setError(null);
    try {
      const res = await authApi.login(values);
      const { user, tokens } = res.data;
      setUser(user);
      setTokens(tokens.access_token, tokens.refresh_token);
      notifications.show({
        title: "Welcome back!",
        message: `Hello, ${user.full_name}`,
        color: "green",
      });
      navigate("/dashboard");
    } catch (err: any) {
      setError(
        err?.response?.data?.message ||
          "Invalid credentials. Please try again.",
      );
    } finally {
      setLoading(false);
    }
  };

  return (
    <Container size={440} my={80}>
      <Title ta="center" fw={900}>
        Welcome back
      </Title>
      <Text c="dimmed" size="sm" ta="center" mt={5}>
        Sign in to your ERP account
      </Text>

      <Paper withBorder shadow="md" p={30} mt={30} radius="md">
        <Group justify="center" mb="md">
          <ThemeIcon size={48} radius="xl" color="erp-green">
            <IconLock size={24} />
          </ThemeIcon>
        </Group>

        {error && (
          <Alert icon={<IconAlertCircle />} color="red" mb="md" radius="md">
            {error}
          </Alert>
        )}

        <form onSubmit={form.onSubmit(handleSubmit)}>
          <Stack>
            <TextInput
              label="Username"
              placeholder="your.username"
              required
              {...form.getInputProps("username")}
            />
            <PasswordInput
              label="Password"
              placeholder="Your password"
              required
              {...form.getInputProps("password")}
            />
            <Anchor component={Link} to="/forgot-password" size="sm" ta="right">
              Forgot password?
            </Anchor>
            <Button type="submit" fullWidth loading={loading} size="md">
              Sign in
            </Button>
          </Stack>
        </form>

        <Divider my="md" label="New here?" labelPosition="center" />
        <Text ta="center" size="sm">
          Don't have an account?{" "}
          <Anchor component={Link} to="/register">
            Register
          </Anchor>
        </Text>
      </Paper>
    </Container>
  );
}
