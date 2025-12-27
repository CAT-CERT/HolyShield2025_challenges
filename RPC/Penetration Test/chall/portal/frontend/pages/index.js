import { useState, useEffect } from 'react';
import Head from 'next/head';
import Link from 'next/link';
import axios from 'axios';

export default function Home() {
  const [posts, setPosts] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchPosts = async () => {
      try {
        const apiUrl = process.env.STRAPI_API_URL || 'http://localhost:1337';
        const response = await axios.get(`${apiUrl}/api/articles`);
        setPosts(response.data.data || []);
      } catch (error) {
        console.error('Failed to fetch posts:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchPosts();
  }, []);

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

  return (
    <>
      <Head>
        <title>Veritas Mobile - Newsletter</title>
        <meta name="generator" content="Next.js 15.2.2" />
        <meta name="description" content="Veritas Mobile Newsletter and Updates" />
      </Head>
      
      <div style={styles.container}>
        <header style={styles.header}>
        <div style={styles.headerContent}>
          <div style={styles.logo}>
            <img src="/logo.png" alt="Veritas Mobile" style={styles.logoImage} />
            <span style={styles.logoText}>Veritas Mobile</span>
          </div>
          <nav style={styles.nav}>
            <a href="#" style={styles.navLink}>Home</a>
            <a href="#" style={styles.navLink}>Services</a>
            <a href="#" style={styles.navLink}>Plans</a>
            <a href="#" style={styles.navLink}>Support</a>
          </nav>
        </div>
      </header>

      <div style={styles.hero}>
        <h1 style={styles.heroTitle}>Latest Updates & News</h1>
        <p style={styles.heroSubtitle}>Stay connected with Veritas Mobile's newest announcements</p>
      </div>

      <main style={styles.main}>
        {posts.length === 0 ? (
          <div style={styles.emptyState}>
            <p>No newsletters available at the moment.</p>
            <p style={styles.emptySubtext}>Check back soon for updates!</p>
          </div>
        ) : (
          <div style={styles.postList}>
            {posts.map((post) => (
              <article key={post.id} style={styles.postCard}>
                <Link href={`/post/${post.id}`} style={styles.postLink}>
                  <div style={styles.postBadge}>Newsletter</div>
                  <h2 style={styles.postTitle}>{post.attributes?.title || 'Untitled'}</h2>
                  <p style={styles.postDescription}>{post.attributes?.description || ''}</p>
                  <div style={styles.postFooter}>
                    <span style={styles.postDate}>
                      {new Date(post.attributes?.createdAt).toLocaleDateString('en-US', { 
                        year: 'numeric', 
                        month: 'long', 
                        day: 'numeric' 
                      })}
                    </span>
                    <span style={styles.readMore}>Read more →</span>
                  </div>
                </Link>
              </article>
            ))}
          </div>
        )}
      </main>

        <footer style={styles.footer}>
          <p style={styles.footerText}>© 2025 Veritas Mobile. All rights reserved.</p>
          <p style={styles.footerSubtext}>Connecting you to what matters most</p>
        </footer>
      </div>
    </>
  );
}

const styles = {
  container: {
    minHeight: '100vh',
    backgroundColor: '#f8f9fa',
    fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif',
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
  header: {
    backgroundColor: '#ffffff',
    borderBottom: '1px solid #e0e0e0',
    boxShadow: '0 2px 4px rgba(0,0,0,0.05)',
  },
  headerContent: {
    maxWidth: '1200px',
    margin: '0 auto',
    padding: '20px 40px',
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
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
  nav: {
    display: 'flex',
    gap: '30px',
  },
  navLink: {
    color: '#495057',
    textDecoration: 'none',
    fontSize: '15px',
    fontWeight: '500',
    transition: 'color 0.2s',
  },
  hero: {
    backgroundColor: 'linear-gradient(135deg, #0056b3 0%, #004494 100%)',
    background: 'linear-gradient(135deg, #0056b3 0%, #004494 100%)',
    color: 'white',
    padding: '60px 40px',
    textAlign: 'center',
  },
  heroTitle: {
    fontSize: '42px',
    fontWeight: '700',
    margin: '0 0 15px 0',
    letterSpacing: '-1px',
  },
  heroSubtitle: {
    fontSize: '18px',
    opacity: '0.95',
    margin: '0',
    fontWeight: '400',
  },
  main: {
    maxWidth: '1000px',
    margin: '0 auto',
    padding: '50px 40px',
  },
  postList: {
    display: 'grid',
    gap: '24px',
  },
  postCard: {
    backgroundColor: '#ffffff',
    borderRadius: '12px',
    overflow: 'hidden',
    boxShadow: '0 2px 8px rgba(0,0,0,0.08)',
    transition: 'transform 0.2s, box-shadow 0.2s',
    cursor: 'pointer',
  },
  postLink: {
    display: 'block',
    padding: '28px',
    textDecoration: 'none',
    color: 'inherit',
  },
  postBadge: {
    display: 'inline-block',
    backgroundColor: '#e3f2fd',
    color: '#0056b3',
    padding: '6px 14px',
    borderRadius: '20px',
    fontSize: '13px',
    fontWeight: '600',
    marginBottom: '16px',
    textTransform: 'uppercase',
    letterSpacing: '0.5px',
  },
  postTitle: {
    fontSize: '24px',
    fontWeight: '700',
    color: '#212529',
    margin: '0 0 12px 0',
    lineHeight: '1.3',
  },
  postDescription: {
    fontSize: '16px',
    color: '#6c757d',
    lineHeight: '1.6',
    margin: '0 0 20px 0',
  },
  postFooter: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    paddingTop: '16px',
    borderTop: '1px solid #f0f0f0',
  },
  postDate: {
    fontSize: '14px',
    color: '#868e96',
  },
  readMore: {
    color: '#0056b3',
    fontSize: '15px',
    fontWeight: '600',
  },
  emptyState: {
    textAlign: 'center',
    padding: '60px 20px',
    color: '#6c757d',
  },
  emptySubtext: {
    marginTop: '10px',
    fontSize: '14px',
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

