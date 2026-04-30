import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { createUser } from '../services/users';
import './FormPage.css';

export default function NewUserPage() {
  const navigate = useNavigate();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  
  const [form, setForm] = useState({
    username: '',
    email: '',
    first_name: '',
    last_name: '',
    password: '',
    role_id: '',
  });

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    setForm(prev => ({ ...prev, [e.target.name]: e.target.value }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    
    if (!form.username || !form.email || !form.password || !form.role_id) {
      setError('Please fill out all required fields.');
      return;
    }
    
    try {
      setLoading(true);
      await createUser(form);
      navigate('/users');
    } catch (err: any) {
      setError(err.message || 'Failed to create user. Make sure the email/username is unique.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="form-root">
      <div className="form-header">
        <div className="form-header__text">
          <h1 className="form-title">Add User</h1>
          <p className="form-subtitle">Create a new system user profile.</p>
        </div>
        <button className="form-btn form-btn--ghost" onClick={() => navigate('/users')}>
          Cancel
        </button>
      </div>

      {error && (
        <div className="form-error">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/></svg>
          {error}
        </div>
      )}

      <form className="form-card" onSubmit={handleSubmit}>
        <div className="form-grid">
          <div className="form-group">
            <label className="form-label">Username *</label>
            <input className="form-input" name="username" value={form.username} onChange={handleChange} required />
          </div>
          <div className="form-group">
            <label className="form-label">Email Address *</label>
            <input className="form-input" type="email" name="email" value={form.email} onChange={handleChange} required />
          </div>
          <div className="form-group">
            <label className="form-label">First Name</label>
            <input className="form-input" name="first_name" value={form.first_name} onChange={handleChange} />
          </div>
          <div className="form-group">
            <label className="form-label">Last Name</label>
            <input className="form-input" name="last_name" value={form.last_name} onChange={handleChange} />
          </div>
          <div className="form-group">
            <label className="form-label">Password *</label>
            <input className="form-input" type="password" name="password" value={form.password} onChange={handleChange} required />
          </div>
          <div className="form-group">
            <label className="form-label">Role *</label>
            <select className="form-input" name="role_id" value={form.role_id} onChange={handleChange} required>
              <option value="">Select a role...</option>
              <option value="1">Admin</option>
              <option value="2">User</option>
            </select>
          </div>
        </div>

        <div className="form-footer">
          <button type="button" className="form-btn form-btn--ghost" onClick={() => navigate('/users')} disabled={loading}>
            Cancel
          </button>
          <button type="submit" className="form-btn form-btn--primary" disabled={loading}>
            {loading ? 'Saving...' : 'Save User'}
          </button>
        </div>
      </form>
    </div>
  );
}
