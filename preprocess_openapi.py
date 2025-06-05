#!/usr/bin/env python3
"""
Preprocess OpenAPI spec to mark datetime fields with proper format.
This script identifies string fields that are likely datetime fields and
adds the appropriate format attribute.
"""

import yaml
import json
import sys
from typing import Dict, Any

# Known datetime field names
DATETIME_FIELDS = {
    'initial_fetch_at', 'created_at', 'updated_at', 'last_fetch_at', 
    'last_seen_at', 'expires_at', 'expire_at', 'lastSyncedAt'
}

def process_properties(properties: Dict[str, Any]) -> Dict[str, Any]:
    """Process properties in a schema and mark datetime fields."""
    for field_name, field_spec in properties.items():
        if (isinstance(field_spec, dict) and 
            field_spec.get('type') == 'string' and
            field_name in DATETIME_FIELDS):
            # Mark as datetime with RFC3339 format
            field_spec['format'] = 'date-time'
            print(f"Marked field '{field_name}' as date-time")
            
            # Add example if it doesn't exist
            if 'example' not in field_spec:
                field_spec['example'] = "2019-11-21T03:45:47.982Z"
    
    return properties

def process_schema(schema: Dict[str, Any]) -> Dict[str, Any]:
    """Recursively process a schema object."""
    if not isinstance(schema, dict):
        return schema
    
    # Process properties
    if 'properties' in schema:
        schema['properties'] = process_properties(schema['properties'])
    
    # Process nested schemas
    for key in ['items', 'additionalProperties']:
        if key in schema:
            schema[key] = process_schema(schema[key])
    
    # Process allOf, oneOf, anyOf
    for key in ['allOf', 'oneOf', 'anyOf']:
        if key in schema:
            schema[key] = [process_schema(s) for s in schema[key]]
    
    return schema

def process_openapi_spec(spec: Dict[str, Any]) -> Dict[str, Any]:
    """Process the entire OpenAPI specification."""
    
    # Process components/schemas
    if 'components' in spec and 'schemas' in spec['components']:
        for schema_name, schema_def in spec['components']['schemas'].items():
            spec['components']['schemas'][schema_name] = process_schema(schema_def)
    
    # Process paths and inline schemas
    if 'paths' in spec:
        for path, path_item in spec['paths'].items():
            for method, operation in path_item.items():
                if not isinstance(operation, dict):
                    continue
                
                # Process request body schemas
                if 'requestBody' in operation:
                    request_body = operation['requestBody']
                    if 'content' in request_body:
                        for content_type, content in request_body['content'].items():
                            if 'schema' in content:
                                content['schema'] = process_schema(content['schema'])
                
                # Process response schemas
                if 'responses' in operation:
                    for status, response in operation['responses'].items():
                        if 'content' in response:
                            for content_type, content in response['content'].items():
                                if 'schema' in content:
                                    content['schema'] = process_schema(content['schema'])
    
    return spec

def main():
    if len(sys.argv) != 3:
        print("Usage: python preprocess_openapi.py <input_file> <output_file>")
        sys.exit(1)
    
    input_file = sys.argv[1]
    output_file = sys.argv[2]
    
    # Read the OpenAPI spec
    with open(input_file, 'r', encoding='utf-8') as f:
        if input_file.endswith('.yaml') or input_file.endswith('.yml'):
            spec = yaml.safe_load(f)
        else:
            spec = json.load(f)
    
    # Process the spec
    processed_spec = process_openapi_spec(spec)
    
    # Write the processed spec
    with open(output_file, 'w', encoding='utf-8') as f:
        if output_file.endswith('.yaml') or output_file.endswith('.yml'):
            yaml.dump(processed_spec, f, default_flow_style=False, sort_keys=False)
        else:
            json.dump(processed_spec, f, indent=2)
    
    print(f"Processed OpenAPI spec written to {output_file}")

if __name__ == "__main__":
    main() 