# Ep. {{episode.number}}: {{episode.title}}

Title
```
{{ episode.title }}
```

Episode slug
```
{{ crate::template::get_episode_slug(episode) }}
```

### For Spotify

```html
{{ spotify_html }}
```

### For YouTube

Content
```text
{{episode.description}}

TRANSCRIPT
{{ crate::template::get_transcript_url(episode, podcast_info) }}

EPISODE LINKS
{%- for link in episode.links %}
{{link.text}}: {{link.href}}
{%- endfor %}

PODCAST LINKS
{%- for link in podcast_info.links %}
{{link.text}}: {{link.href}}
{%- endfor %}

OUTLINE
{%- for entry in outline %}
{{entry.time_code }} - {{entry.text}}
{%- endfor %}
```


