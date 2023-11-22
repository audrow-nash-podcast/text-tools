# Ep. {{episode.number}}: {{episode.title}}

Title
```
{{ episode.title }}
```

Episode slug
```
{{ crate::get_episode_slug(episode) }}
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
{{ crate::get_transcript_url(episode, podcast_info) }}

EPISODE LINKS
{%- for link in episode.links %}
{{link.text}}: {{link.href}}
{%- endfor %}

PODCAST LINKS
{%- for link in podcast_info.links %}
{{link.text}}: {{link.href}}
{%- endfor %}

OUTLINE
{%- for time_code in time_codes %}
{{time_code.timestamp }} - {{time_code.text}}
{%- endfor %}
```


