# getprojectconfig

```
function getProjectConfig(): Promise<ProjectConfig>;

```

Gets the project configuration for the current project.

## 

[​](#returns)

Returns

`Promise`<`ProjectConfig`\> The project configuration.

## 

[​](#example)

Example

```
const result = await getProjectConfig();
console.log("Project name:", result.name);

```