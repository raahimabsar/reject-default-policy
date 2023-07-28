
# Kubewarden policy reject-default-policy

## Description

This policy will reject pods that are created in "defualt" namespace. If
the pod to be validated is created in a different namespace, or if a different type
of resource is evaluated, it will be accepted.

## Settings

This policy has no configurable settings. This policy is designed to reject only the 
pods created in default namespace.

## License

```
Copyright (C) 2021 Raahim_Absar

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
