import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { createCustomer } from '../services/customers';
import './FormPage.css';

export default function NewCustomerPage() {
  const navigate = useNavigate();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  
  const [form, setForm] = useState({
    name: '',
    email: '',
    phone: '',
    address: '',
    city: '',
    country: '',
    tax_number: '',
    credit_limit: '',
  });

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setForm(prev => ({ ...prev, [e.target.name]: e.target.value }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    
    if (!form.name || !form.email) {
      setError('Please fill out all required fields.');
      return;
    }
    
    try {
      setLoading(true);
      await createCustomer({
        ...form,
        credit_limit: Number(form.credit_limit) || 0,
      });
      navigate('/customers');
    } catch (err: any) {
      setError(err.message || 'Failed to create customer.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="form-root">
      <div className="form-header">
        <div className="form-header__text">
          <h1 className="form-title">Add Customer</h1>
          <p className="form-subtitle">Create a new customer profile in the system.</p>
        </div>
        <button className="form-btn form-btn--ghost" onClick={() => navigate('/customers')}>
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
            <label className="form-label">Full Name *</label>
            <input className="form-input" name="name" value={form.name} onChange={handleChange} required />
          </div>
          <div className="form-group">
            <label className="form-label">Email Address *</label>
            <input className="form-input" type="email" name="email" value={form.email} onChange={handleChange} required />
          </div>
          
          <div className="form-group">
            <label className="form-label">Phone Number</label>
            <input className="form-input" type="tel" name="phone" value={form.phone} onChange={handleChange} />
          </div>
          <div className="form-group">
            <label className="form-label">Tax Number (VAT/NTN)</label>
            <input className="form-input" name="tax_number" value={form.tax_number} onChange={handleChange} />
          </div>

          <div className="form-group form-group--full">
            <label className="form-label">Street Address</label>
            <input className="form-input" name="address" value={form.address} onChange={handleChange} />
          </div>

          <div className="form-group">
            <label className="form-label">City</label>
            <input className="form-input" name="city" value={form.city} onChange={handleChange} />
          </div>
          <div className="form-group">
            <label className="form-label">Country</label>
            <input className="form-input" name="country" value={form.country} onChange={handleChange} />
          </div>

          <div className="form-group form-group--full">
            <label className="form-label">Credit Limit ($)</label>
            <input className="form-input" type="number" step="0.01" name="credit_limit" value={form.credit_limit} onChange={handleChange} placeholder="0.00" />
          </div>
        </div>

        <div className="form-footer">
          <button type="button" className="form-btn form-btn--ghost" onClick={() => navigate('/customers')} disabled={loading}>
            Cancel
          </button>
          <button type="submit" className="form-btn form-btn--primary" disabled={loading}>
            {loading ? 'Saving...' : 'Save Customer'}
          </button>
        </div>
      </form>
    </div>
  );
}
