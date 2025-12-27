import Head from 'next/head';
import Link from 'next/link';

export default function Forbidden() {
  return (
    <>
      <Head>
        <title>403 Forbidden - Veritas Mobile</title>
      </Head>

      <div style={{
        minHeight: '100vh',
        background: 'linear-gradient(135deg, #f093fb 0%, #f5576c 100%)',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        fontFamily: 'system-ui, -apple-system, sans-serif'
      }}>
        <div style={{
          background: 'white',
          padding: '3rem',
          borderRadius: '12px',
          boxShadow: '0 20px 60px rgba(0,0,0,0.3)',
          maxWidth: '500px',
          width: '90%',
          textAlign: 'center'
        }}>
          <div style={{
            fontSize: '6rem',
            fontWeight: 'bold',
            color: '#f5576c',
            marginBottom: '1rem',
            lineHeight: '1'
          }}>
            403
          </div>

          <h1 style={{
            fontSize: '2rem',
            color: '#333',
            marginBottom: '1rem'
          }}>
            Access Forbidden
          </h1>

          <p style={{
            color: '#666',
            fontSize: '1rem',
            marginBottom: '2rem',
            lineHeight: '1.6'
          }}>
            You don't have permission to access this page. Please contact your administrator if you believe this is an error.
          </p>

          <div style={{
            background: '#fee',
            border: '1px solid #fcc',
            borderRadius: '8px',
            padding: '1rem',
            marginBottom: '2rem',
            color: '#c33',
            fontSize: '0.9rem'
          }}>
            🔒 <strong>Authentication Required</strong>
            <br />
            Valid credentials are needed to access this resource.
          </div>

          <Link href="/" style={{
            display: 'inline-block',
            padding: '0.75rem 2rem',
            background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
            color: 'white',
            textDecoration: 'none',
            borderRadius: '6px',
            fontWeight: '600',
            fontSize: '1rem',
            transition: 'transform 0.2s'
          }}>
            Return to Home
          </Link>
        </div>
      </div>
    </>
  );
}

