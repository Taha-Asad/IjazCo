import { TextInput, PasswordInput, Button, Stack, Title, Text, Group } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useNavigate, Link } from "react-router-dom";
import { useMutation } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { authApi } from "../../api/auth";

export function RegisterPage() {
  const navigate = useNavigate();
  const registerMutation = useMutation({
    mutationFn: authApi.register,
    onSuccess: () => {
      notifications.show({
        title: "Registered",
        message: "Account created successfully. Please login.",
        color: "green",
      });
      navigate("/login");
    },
    onError: (error: any) => {
      notifications.show({
        title: "Registration failed",
        message: error.message || "An error occurred.",
        color: "red",
      });
    },
  });

  const form = useForm({
    initialValues: {
      full_name: "",
      email: "",
      password: "",
      confirm_password: "",
    },
    validate: {
      full_name: (v) => (!v ? "Name required" : null),
      email: (v) =>
        !v || !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(v) ? "Invalid email" : null,
      password: (v) =>
        v && v.length < 8 ? "Password must be at least 8 characters" : null,
      confirm_password: (v, values) =>
        v !== values.password ? "Passwords do not match" : null,
    },
  });

  return (
    <form onSubmit={form.onSubmit((v) => registerMutation.mutateAsync(v))}>
      <Stack>
        <Title order={2} ta="center">Create Account</Title>
        <Text c="dimmed" size="sm" ta="center">
          Already have an account?{" "}
          <Text component={Link} to="/login" span c="blue" fw={500}>
            Sign in
          </Text>
        </Text>
        <TextInput
          label="Full Name"
          placeholder="John Doe"
          required
          {...form.getInputProps("full_name")}
        />
        <TextInput
          label="Email"
          placeholder="john@example.com"
          required
          {...form.getInputProps("email")}
        />
        <PasswordInput
          label="Password"
          placeholder="••••••••"
          required
          {...form.getInputProps("password")}
        />
        <PasswordInput
          label="Confirm Password"
          placeholder="••••••••"
          required
          {...form.getInputProps("confirm_password")}
        />
        <Button type="submit" loading={registerMutation.isPending}>
          Register
        </Button>
      </Stack>
    </form>
  );
}
