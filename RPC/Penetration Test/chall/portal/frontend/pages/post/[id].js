import { useState, useEffect } from 'react';
import { useRouter } from 'next/router';
import Link from 'next/link';
import axios from 'axios';

export default function Post() {
  const router = useRouter();
  const { id } = router.query;
  const [post, setPost] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!id) return;

    const fetchPost = async () => {
      try {
        const apiUrl = process.env.STRAPI_API_URL || 'http://localhost:1337';
        const response = await axios.get(`${apiUrl}/api/articles/${id}`);
        setPost(response.data.data);
      } catch (error) {
        console.error('Failed to fetch post:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchPost();
  }, [id]);

  if (loading) {
    return (
      <div style={styles.container}>
        <div style={styles.loadingContainer}>
          <div style={styles.spinner}></div>
          <p>Loading...</p>
        </div>
      </div>
    );
  }

  if (!post) {
    return (
      <div style={styles.pageContainer}>
        <header style={styles.header}>
          <div style={styles.headerContent}>
            <div style={styles.logo}>
              <img src="/logo.png" alt="Veritas Mobile" style={styles.logoImage} />
              <span style={styles.logoText}>Veritas Mobile</span>
            </div>
          </div>
        </header>
        <div style={styles.container}>
          <h1>Newsletter not found</h1>
          <Link href="/" style={styles.backLink}>← Back to newsletters</Link>
        </div>
      </div>
    );
  }

  return (
    <div style={styles.pageContainer}>
      <header style={styles.header}>
        <div style={styles.headerContent}>
          <div style={styles.logo}>
            <img src="/logo.svg" alt="Veritas Mobile" style={styles.logoImage} onError={(e) => { e.target.onerror = null; e.target.src = '/logo.png'; }} />
            <span style={styles.logoText}>Veritas Mobile</span>
          </div>
        </div>
      </header>

      <div style={styles.container}>
        <Link href="/" style={styles.backLink}>← Back to all newsletters</Link>
        
        <article style={styles.article}>
          <div style={styles.badge}>Newsletter</div>
          <h1 style={styles.title}>{post.attributes?.title || 'Untitled'}</h1>
          <div style={styles.meta}>
            <span style={styles.metaItem}>
              📅 {new Date(post.attributes?.createdAt).toLocaleDateString('en-US', { 
                year: 'numeric', 
                month: 'long', 
                day: 'numeric' 
              })}
            </span>
            <span style={styles.metaItem}>📰 Veritas Mobile News</span>
          </div>
          <div style={styles.contentBox}>
            <div style={styles.content}>
              {post.attributes?.content || post.attributes?.description || 'No content available'}
            </div>
          </div>
        </article>
      </div>

      <footer style={styles.footer}>
        <p style={styles.footerText}>© 2025 Veritas Mobile. All rights reserved.</p>
        <p style={styles.footerSubtext}>Connecting you to what matters most</p>
      </footer>
    </div>
  );
}

const styles = {
  pageContainer: {
    minHeight: '100vh',
    backgroundColor: '#f8f9fa',
    fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif',
  },
  header: {
    backgroundColor: '#ffffff',
    borderBottom: '1px solid #e0e0e0',
    boxShadow: '0 2px 4px rgba(0,0,0,0.05)',
    marginBottom: '40px',
  },
  headerContent: {
    maxWidth: '1000px',
    margin: '0 auto',
    padding: '20px 40px',
  },
  logo: {
    display: 'flex',
    alignItems: 'center',
    gap: '12px',
    fontSize: '24px',
    fontWeight: '700',
    color: '#0056b3',
  },
  logoImage: {
    height: '40px',
    width: 'auto',
  },
  logoText: {
    letterSpacing: '-0.5px',
  },
  loadingContainer: {
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    justifyContent: 'center',
    minHeight: '100vh',
    color: '#0056b3',
  },
  spinner: {
    width: '50px',
    height: '50px',
    border: '4px solid #e3f2fd',
    borderTop: '4px solid #0056b3',
    borderRadius: '50%',
    animation: 'spin 1s linear infinite',
  },
  container: {
    maxWidth: '900px',
    margin: '0 auto',
    padding: '20px 40px 80px',
  },
  backLink: {
    color: '#0056b3',
    textDecoration: 'none',
    display: 'inline-flex',
    alignItems: 'center',
    marginBottom: '30px',
    fontSize: '15px',
    fontWeight: '600',
    transition: 'color 0.2s',
  },
  article: {
    backgroundColor: '#ffffff',
    borderRadius: '12px',
    padding: '50px',
    boxShadow: '0 2px 8px rgba(0,0,0,0.08)',
  },
  badge: {
    display: 'inline-block',
    backgroundColor: '#e3f2fd',
    color: '#0056b3',
    padding: '8px 16px',
    borderRadius: '20px',
    fontSize: '13px',
    fontWeight: '600',
    marginBottom: '20px',
    textTransform: 'uppercase',
    letterSpacing: '0.5px',
  },
  title: {
    fontSize: '36px',
    fontWeight: '700',
    color: '#212529',
    margin: '0 0 20px 0',
    lineHeight: '1.2',
  },
  meta: {
    display: 'flex',
    gap: '20px',
    paddingBottom: '30px',
    borderBottom: '2px solid #f0f0f0',
    marginBottom: '30px',
  },
  metaItem: {
    color: '#868e96',
    fontSize: '14px',
  },
  contentBox: {
    lineHeight: '1.8',
  },
  content: {
    fontSize: '17px',
    color: '#495057',
    whiteSpace: 'pre-wrap',
    lineHeight: '1.8',
  },
  footer: {
    backgroundColor: '#212529',
    color: '#ffffff',
    textAlign: 'center',
    padding: '40px 20px',
    marginTop: '60px',
  },
  footerText: {
    margin: '0 0 8px 0',
    fontSize: '14px',
  },
  footerSubtext: {
    margin: '0',
    fontSize: '13px',
    opacity: '0.7',
  },
};

