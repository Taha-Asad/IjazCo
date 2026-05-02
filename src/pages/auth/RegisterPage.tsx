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
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { z } from "zod";
import { zodResolver } from "mantine-form-zod-resolver";
import { useState } from "react";
import { useNavigate, Link } from "react-router-dom";
import { notifications } from "@mantine/notifications";
import { authApi } from "../../api/auth";
import { useAuthStore } from "../../store/authStore";

const schema = z
  .object({
    full_name: z.string().min(2, "Full name required"),
    username: z.string().min(3, "Username min 3 characters"),
    email: z.string().email("Valid email required"),
    password: z.string().min(8, "Password min 8 characters"),
    confirm_password: z.string(),
  })
  .refine((d) => d.password === d.confirm_password, {
    message: "Passwords do not match",
    path: ["confirm_password"],
  });

export function RegisterPage() {
  const navigate = useNavigate();
  const { setUser, setTokens } = useAuthStore();
  const [loading, setLoading] = useState(false);

  const form = useForm({
    validate: zodResolver(schema),
    initialValues: {
      full_name: "",
      username: "",
      email: "",
      password: "",
      confirm_password: "",
    },
  });

  const handleSubmit = async (values: typeof form.values) => {
    setLoading(true);
    try {
      const res = await authApi.register({
        full_name: values.full_name,
        username: values.username,
        email: values.email,
        password: values.password,
      });
      setUser(res.data.user);
      setTokens(res.data.tokens.access_token, res.data.tokens.refresh_token);
      notifications.show({
        title: "Welcome!",
        message: "Account created.",
        color: "green",
      });
      navigate("/dashboard");
    } catch (err: any) {
      notifications.show({
        title: "Error",
        message: err?.response?.data?.message || "Registration failed.",
        color: "red",
      });
    } finally {
      setLoading(false);
    }
  };

  return (
    <Container size={440} my={80}>
      <Title ta="center" fw={900}>
        Create Account
      </Title>
      <Text c="dimmed" size="sm" ta="center" mt={5}>
        Already have an account?{" "}
        <Anchor component={Link} to="/login">
          Sign in
        </Anchor>
      </Text>
      <Paper withBorder shadow="md" p={30} mt={30} radius="md">
        <form onSubmit={form.onSubmit(handleSubmit)}>
          <Stack>
            <TextInput
              label="Full Name"
              placeholder="John Doe"
              required
              {...form.getInputProps("full_name")}
            />
            <TextInput
              label="Username"
              placeholder="john.doe"
              required
              {...form.getInputProps("username")}
            />
            <TextInput
              label="Email"
              placeholder="john@example.com"
              required
              {...form.getInputProps("email")}
            />
            <PasswordInput
              label="Password"
              required
              {...form.getInputProps("password")}
            />
            <PasswordInput
              label="Confirm Password"
              required
              {...form.getInputProps("confirm_password")}
            />
            <Button type="submit" fullWidth loading={loading}>
              Create Account
            </Button>
          </Stack>
        </form>
      </Paper>
    </Container>
  );
}
