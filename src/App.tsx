import { useEffect, useState } from "react";
import { AppRouter } from "./router";
import { useAuthStore } from "./store/authStore";
import { authApi } from "./api/auth";

function App() {
  const { isAuthenticated, setUser, logout } = useAuthStore();
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (isAuthenticated) {
      // Always refresh user data from API to ensure company_id is present
      authApi
        .me()
        .then((user) => {
          setUser(user);
          setLoading(false);
        })
        .catch(() => {
          logout();
          setLoading(false);
        });
    } else {
      setLoading(false);
    }
  }, []);

  if (loading) {
    return (
      <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100vh' }}>
        Loading...
      </div>
    );
  }

  return <AppRouter />;
}

export default App;
