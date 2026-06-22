export const divider = async ({ node }) => {
  const { width, border } = node.fields.style

  return `<div style="width:${width};border-bottom:${border};height:1px"></div>`
}
