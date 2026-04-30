import { useEffect } from "react";
import { AppRouter } from "./router";
import { useAuthStore } from "./store/authStore";
import { authApi } from "./api/auth";

function App() {
  const { isAuthenticated, setUser, logout } = useAuthStore();

  useEffect(() => {
    if (isAuthenticated) {
      authApi
        .me()
        .then((res) => setUser(res.data))
        .catch(() => logout());
    }
  }, []);

  return <AppRouter />;
}

export default App;
