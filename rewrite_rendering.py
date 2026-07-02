import re

with open('src/rendering.rs', 'r') as f:
    content = f.read()

# Camera2dBundle
content = content.replace('Camera2dBundle::default()', 'Camera2d')

# SpriteBundle
content = re.sub(
    r'SpriteBundle\s*\{\s*sprite:\s*(.*?),\s*transform:\s*(.*?),\s*\.\.default\(\)\s*\}',
    r'\1,\n        \2',
    content,
    flags=re.DOTALL
)

# NodeBundle
def repl_node_bundle(m):
    body = m.group(1)
    # Extract style
    style_match = re.search(r'style:\s*Style\s*\{([\s\S]*?)\},', body)
    bg_match = re.search(r'background_color:\s*(BackgroundColor\([^)]+\)),', body)
    
    res = 'Node {'
    if style_match:
        style_inner = style_match.group(1).strip()
        if style_inner != '..default()':
            # Remove trailing ..default() inside style if present
            style_inner = re.sub(r',\s*\.\.default\(\)\s*$', '', style_inner)
            res += '\n' + style_inner + ',\n..default()'
        else:
            res += '..default()'
    else:
        res += '..default()'
    res += '}'
    
    if bg_match:
        res += ',\n        ' + bg_match.group(1)
        
    return res

content = re.sub(r'NodeBundle\s*\{([\s\S]*?)(?:\.\.default\(\)\s*)?\}', repl_node_bundle, content)

# TextBundle::from_section
def repl_text_bundle_from_section(m):
    text_val = m.group(1)
    style_inner = m.group(2)
    # style_inner has font_size, color, etc.
    color_match = re.search(r'color:\s*([^,]+),', style_inner)
    font_size_match = re.search(r'font_size:\s*([^,]+),', style_inner)
    
    color = color_match.group(1) if color_match else "Color::WHITE"
    font_size = font_size_match.group(1) if font_size_match else "12.0"
    
    return f'Text::new({text_val}),\n        TextFont {{\n            font_size: {font_size},\n            ..default()\n        }},\n        TextColor({color})'

content = re.sub(r'TextBundle::from_section\(\s*(.*?),\s*TextStyle\s*\{([\s\S]*?)\}\s*,\s*\)', repl_text_bundle_from_section, content)

# TextBundle
def repl_text_bundle(m):
    body = m.group(1)
    text_match = re.search(r'text:\s*Text::from_section\(\s*(.*?),\s*TextStyle\s*\{([\s\S]*?)\}\s*,\s*\),', body)
    if text_match:
        text_val = text_match.group(1)
        style_inner = text_match.group(2)
        color_match = re.search(r'color:\s*([^,]+),', style_inner)
        font_size_match = re.search(r'font_size:\s*([^,]+),', style_inner)
        color = color_match.group(1) if color_match else "Color::WHITE"
        font_size = font_size_match.group(1) if font_size_match else "12.0"
        
        node_match = re.search(r'style:\s*Style\s*\{([\s\S]*?)\},', body)
        node_str = "Node { ..default() }"
        if node_match:
            node_inner = node_match.group(1).strip()
            node_inner = re.sub(r',\s*\.\.default\(\)\s*$', '', node_inner)
            node_str = f"Node {{\n{node_inner},\n..default()\n}}"
            
        bg_match = re.search(r'background_color:\s*(BackgroundColor\([^)]+\)),', body)
        vis_match = re.search(r'visibility:\s*(Visibility::[^,]+),', body)
        
        res = f'Text::new({text_val}),\n        TextFont {{\n            font_size: {font_size},\n            ..default()\n        }},\n        TextColor({color}),\n        {node_str}'
        if bg_match:
            res += ',\n        ' + bg_match.group(1)
        if vis_match:
            res += ',\n        ' + vis_match.group(1)
        return res
    return m.group(0)

content = re.sub(r'TextBundle\s*\{([\s\S]*?)(?:\.\.default\(\)\s*)?\}', repl_text_bundle, content)

# Update Query<&mut Style> to Query<&mut Node>
content = content.replace('mut hud_container: Query<&mut Style, With<HudContainer>>', 'mut hud_container: Query<&mut Node, With<HudContainer>>')

# Replace text.sections[0].value with *text = Text::new(...) or text.0
content = re.sub(r'text\.sections\[0\]\.value\s*=\s*(.*?;)', r'text.0 = \1', content)

with open('src/rendering.rs', 'w') as f:
    f.write(content)
