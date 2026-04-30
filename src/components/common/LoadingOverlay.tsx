import { Center, Loader, Stack, Text, Box } from "@mantine/core";

interface LoadingOverlayProps {
  visible: boolean;
  message?: string;
  fullScreen?: boolean;
}

export function LoadingOverlay({
  visible,
  message = "Loading...",
  fullScreen = false,
}: LoadingOverlayProps) {
  if (!visible) return null;

  const content = (
    <Stack align="center" gap="sm">
      <Loader size="lg" color="erp-green" />
      <Text size="sm" c="dimmed">
        {message}
      </Text>
    </Stack>
  );

  if (fullScreen) {
    return (
      <Box
        style={{
          position: "fixed",
          inset: 0,
          zIndex: 9999,
          background: "rgba(255,255,255,0.8)",
          backdropFilter: "blur(4px)",
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
        }}
      >
        {content}
      </Box>
    );
  }

  return <Center py={60}>{content}</Center>;
}
