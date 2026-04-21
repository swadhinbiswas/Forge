import rss from '@astrojs/rss';
import { getCollection } from 'astro:content';

export async function GET(context) {
  const docs = await getCollection('docs');
  // Only include items in the blog directory (excluding index if any exist)
  const blogPosts = docs.filter(
    (doc) => doc.id.startsWith('blog/') && !doc.id.endsWith('index.md') && !doc.id.endsWith('index.mdx')
  );

  return rss({
    title: 'ForgeDesk Blog Updates',
    description: 'Latest news, releases, and guides for the ForgeDesk Python framework.',
    site: context.site || 'https://docs.forgedesk.dev',
    items: blogPosts.map((post) => ({
      title: post.data.title,
      // Default to current date if missing so validation passes
      pubDate: post.data.date || new Date(), 
      description: post.data.description || 'ForgeDesk Update',
      // Generating standard static link
      link: `/${post.slug}/`,
    })),
    customData: `<language>en-us</language>`,
  });
}
