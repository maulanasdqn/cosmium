import { createFormHook } from '@tanstack/react-form'

import { fieldContext, formContext } from './form-context'
import { FormRoot } from './form-root'
import { LinesField } from './lines-field'
import { NumberField } from './number-field'
import { SelectField } from './select-field'
import { SubmitButton } from './submit-button'
import { SwitchField } from './switch-field'
import { TextareaField } from './textarea-field'
import { TextField } from './text-field'

export const { useAppForm, withForm } = createFormHook({
  fieldContext,
  formContext,
  fieldComponents: {
    TextField,
    TextareaField,
    NumberField,
    SwitchField,
    SelectField,
    LinesField,
  },
  formComponents: {
    FormRoot,
    SubmitButton,
  },
})

export type { TSelectOption } from './select-field'
