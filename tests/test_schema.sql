USE [test]
GO
/****** Object:  Table [dbo].[test]    Script Date: 11.06.2016 09:49:25 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
SET ANSI_PADDING ON
GO
CREATE TABLE [dbo].[test](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[col_varchar_50] [varchar](50) NULL,
	[col_int] [int] NULL,
	[col_guid] [uniqueidentifier] NULL,
	[col_char] [char](2) NULL,
	[col_text] [text] NULL,
	[col_binary] [binary](50) NULL,
	[col_datetime8] [datetime] NULL,
	[col_float] [float] NULL,
	[col_money] [money] NULL,
	[col_bit] [bit] NULL,
	[col_ntext] [ntext] NULL
) ON [PRIMARY] TEXTIMAGE_ON [PRIMARY]

GO
SET ANSI_PADDING OFF
GO
/****** Object:  Table [dbo].[test_not_nullable]    Script Date: 11.06.2016 09:49:25 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
SET ANSI_PADDING ON
GO
CREATE TABLE [dbo].[test_not_nullable](
	[col_varchar_50] [varchar](50) NOT NULL,
	[col_int] [int] NOT NULL,
	[col_guid] [uniqueidentifier] NOT NULL,
	[col_bit] [bit] NOT NULL,
	[col_float] [float] NOT NULL,
	[col_nvarchar_50] [nvarchar](50) NOT NULL,
	[col_money4] [smallmoney] NOT NULL,
	[col_money8] [money] NOT NULL,
	[col_datetime4] [smalldatetime] NOT NULL,
	[col_datetime8] [datetime] NOT NULL,
	[col_decimal] [decimal](4, 2) NOT NULL,
	[col_numeric] [numeric](18, 0) NOT NULL,
	[col_varbinary] [varbinary](50) NOT NULL,
	[col_nchar] [nchar](10) NOT NULL,
	[col_image] [image] NULL
) ON [PRIMARY] TEXTIMAGE_ON [PRIMARY]

GO
SET ANSI_PADDING OFF
GO
/****** Object:  Table [dbo].[test_v73]    Script Date: 11.06.2016 09:49:25 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[test_v73](
	[id_test] [int] IDENTITY(1,1) NOT NULL,
	[col_datetime2] [datetime2](7) NOT NULL,
	[col_date] [date] NOT NULL,
	[col_time] [time](7) NOT NULL,
	[col_datetime_offset] [datetimeoffset](7) NULL
) ON [PRIMARY]

GO
