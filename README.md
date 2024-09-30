# Installation

To use these Mason templates in your project, follow these steps:

1. Ensure you have the Mason CLI installed. If not, install it using:

   ```bash
   dart pub global activate mason_cli
   ```

2. In your project's root directory, initialize Mason (if you haven't already):

   ```bash
   mason init
   ```

3. Add the templates from this repository to your project:

   ```bash
   mason add body --git-url https://github.com/DivinaVia/backend_template.git --git-path templates/body
   mason add default --git-url https://github.com/DivinaVia/backend_template.git --git-path templates/no_param
   mason add param --git-url https://github.com/DivinaVia/backend_template.git --git-path templates/param
   ```

4. After adding the templates, you can verify if they were installed correctly:

   ```bash
   mason list
   ```

   You should see `body`, `no_param`, and `param` in the list of available templates.

5. Now you can use the templates in your project. For example:

   ```bash
   mason make body_endpoint
   mason make no_param_endpoint
   mason make param_endpoint
   ```

   Replace `example` with the desired name for your generated component or file.

Note: If you make changes to the templates in the repository, users can update to the latest version using:

```
mason upgrade
```

This will update all installed templates to their latest versions.